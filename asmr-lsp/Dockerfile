FROM rust:1.67 as builder

# Install dependencies and build targets
RUN cargo install wasm-pack
RUN rustup target add wasm32-unknown-unknown

# Build the asmr-lsp source code and output the wasm to the vscode_ext/src/asmr folder
WORKDIR /mnt/asmr/asmr-lsp
ENTRYPOINT [ "/bin/bash", "-c", "wasm-pack build --target nodejs --release --out-dir /mnt/vscode_ext/asmr" ]
