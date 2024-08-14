FROM rust:latest
WORKDIR /weensyos

# Install dependencies
RUN apt-get update && \
    apt-get install -y curl qemu-system-x86 && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    rustup toolchain install nightly && \
    rustup default nightly && \
    rustup component add llvm-tools-preview && \
    rustup component add rust-src --toolchain nightly

# Install bootimage tool
# TODO: Make custom bootloader (look RedoxOS)
RUN echo "Installing bootimage tool..." && \
    cargo install bootimage

COPY starter-code-rust/ .

# Create an executable
# TODO: Fix seg faults
RUN echo "Building the project..." && \
    cargo build --release

CMD ["cargo", "run"]
