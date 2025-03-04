# Gunakan image dasar Rust
FROM rust:latest

# Set working directory di dalam container
WORKDIR /usr/src/app

# Salin file dari host ke dalam container
COPY . .

# Build proyek Rust
RUN cargo build --release

# Tentukan perintah untuk menjalankan aplikasi
CMD ["./target/release/cpuScanRust"]
