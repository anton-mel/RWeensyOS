// DRAFT v.1

use x86_64::structures::paging::OffsetPageTable;
use crate::{writeln_at, clear_screen, write_at};
use crate::vga_buffer::{BUFFER_WIDTH, Color};
use alloc::format;

const OFFSET: usize = 3;
const ADDRESS_WIDTH: usize = 10;
const PAGE_BLOCK_SIZE: usize = 64;
const PAGESIZE: usize = 4096; // Extract from the bootloader
const MEMSIZE_PHYSICAL: usize = PAGESIZE * PAGE_BLOCK_SIZE * 8;
const MEMSIZE_VIRTUAL: usize = PAGESIZE * PAGE_BLOCK_SIZE * 8;
// Extract MEMSIZE_PHYSICAL from the bootloader


#[repr(u16)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
enum MemStateColor {
    // - The lower 8 bits represent the text character (ASCII value).
    // - The upper 8 bits represent the color value.
    Kernel = ('K' as u16) | ((Color::Red as u16) << 8),
    Free = ('.' as u16) | ((Color::LightGray as u16) << 8),
    Reserved = ('R' as u16) | ((Color::LightGray as u16) << 8),
    Shared = ('S' as u16) | ((Color::White as u16) << 8),
}


// ---------------------------------------------------------------
// Placeholders [REQUIRED FOR AN UPDATE]
const PTE_U: usize = 0x0040;
impl Color {
    fn from_u8(value: u8) -> Color {
        // Example implementation: map u8 values to specific Color variants
        match value {
            0x07 => Color::LightGray, // Example mapping
            _ => Color::Black, // Default or unknown color
        }
    }
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct PageInfo {
    owner: usize,
    refcount: usize,
}
#[allow(dead_code)]
struct Vamapping {
    pn: usize,
    pa: usize,
    perm: usize,
}
static PAGEINFO: [PageInfo; MEMSIZE_PHYSICAL / PAGESIZE] = [PageInfo { owner: 0, refcount: 0 }; MEMSIZE_PHYSICAL / PAGESIZE];
// ---------------------------------------------------------------


//  _________________________________________________________________
// |              ________ PHYSICAL/VIRTUAL MEMORY ______________    |
// |    0x040000 |                                    ∧          |   |
// |    0x080000 |                                    |          |   |
// |    0x0C0000 |                                    8          |   |
// |    0x100000 |<--------------------64-------------|--------->|   |
// |    0x140000 |                                    |          |   |
// |    0x180000 |                                    |          |   |
// |    0x1C0000 |____________________________________∨__________|   |
// |<----------------------------------80--------------------------->|
//  <3> <--10--> <------------------- ... ----------------------> <3>

// memshow_physical
//    Draw a picture of physical memory on the CGA console.
pub fn memshow_physical() {
    clear_screen!();

    writeln_at!(32, 0, Color::White, "PHYSICAL MEMORY");

    let mut x = 0; // VGA buffer X
    let mut y = 0; // VGA buffer Y

    for (pn, page) in PAGEINFO.iter().enumerate() {
        // Memory address
        if pn % PAGE_BLOCK_SIZE == 0 {
            y += 1; // new line
            x = OFFSET;
            let address = format!("0x{:06X}", pn * PAGESIZE);
            writeln_at!(x, y, Color::White, &address);
            x += ADDRESS_WIDTH;
        }

        // TODO: Define based on PAGEINFO
        let color = match page.owner {
            0 => Color::LightGray, // Free page
            _ => Color::Red,       // Kernel page
        };

        // Page status
        if x < BUFFER_WIDTH {
            write_at!(x, y, '.', color);
            x += 1;
        }
    }
}

// memshow_virtual(pagetable, name)
//    Draw a picture of the virtual memory map `pagetable` (named `name`) on
//    the CGA console.
pub fn memshow_virtual(_pagetable: &OffsetPageTable<'_>, name: &str) {
    const PADDING_TOP: usize = 10; // avoid overlap

    let header = format!("VIRTUAL ADDRESS SPACE FOR {}", name);
    writeln_at!(26, PADDING_TOP, Color::White, &header);

    let mut x = 0; // VGA buffer X
    let mut y = PADDING_TOP; // VGA buffer Y (start below the header)

    for va in (0..MEMSIZE_VIRTUAL).step_by(PAGESIZE) {
        // Print address only for every 64 pages
        if (va / PAGESIZE) % 64 == 0 {
            y += 1; // new line
            x = OFFSET;
            let address = format!("0x{:06X}", va);
            writeln_at!(x, y, Color::White, &address);
            x += ADDRESS_WIDTH;
        }

        let vam = Vamapping {
            pn: (va / PAGESIZE) % PAGE_BLOCK_SIZE, // Use a simple pattern for page number
            pa: va, // Physical address (just for illustration)
            perm: if va % 2 == 0 { PTE_U } else { 0 }, // Toggle user access permission
        };
        let page = &PAGEINFO[vam.pn];

        let color = get_page_color(page.owner, page.refcount, vam.perm & PTE_U != 0);

        // Page status
        if x < BUFFER_WIDTH {
            write_at!(x, y, '.', color);
            x += 1;
        }
    }
}

// Map page owner numbers to colors
fn get_page_color(owner: usize, refcount: usize, is_user: bool) -> Color {
    let base_color = match owner {
        0 => Color::LightGray, // Free page
        _ => Color::Red,       // Kernel page
    };

    let color = if is_user {
        // Invert colors
        let inverted_color = (base_color as u8) ^ 0x07;
        Color::from_u8(inverted_color)
    } else {
        base_color
    };

    if refcount > 1 {
        // Darker color for shared pages
        let darker_color = (color as u8) & 0x77;
        Color::from_u8(darker_color)
    } else {
        color
    }
}

