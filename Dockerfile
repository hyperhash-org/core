FROM rust:1-bookworm AS build
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release || true
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd -m hh
COPY --from=build /app/target/release/hh-core /usr/local/bin/hh-core
USER hh
ENV RUST_LOG=info PORT=8800
EXPOSE 8800
CMD ["/usr/local/bin/hh-core"]
