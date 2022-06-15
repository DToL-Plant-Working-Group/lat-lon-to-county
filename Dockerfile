FROM rust
RUN apt install -y pkg-config

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

# Run the binary
ENTRYPOINT ["./target/release/geodojo_county"]
