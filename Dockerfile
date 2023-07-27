FROM ghcr.io/rust-lang/rust:nightly as rust
LABEL org.opencontainers.image.source https://github.com/valeralabs/web
ENV PKG_CONFIG_ALLOW_CROSS=1

FROM rust AS builder 
RUN cargo install cargo-leptos
RUN rustup target add wasm32-unknown-unknown
WORKDIR /usr/src/web
RUN echo "fn main() {}" > dummy.rs
RUN mkdir src && touch src/lib.rs
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --package=web --bin=web --target-dir=target/server --no-default-features --features=ssr --release
RUN cargo build --package=web --lib --target-dir=target/front --target=wasm32-unknown-unknown --no-default-features --features=hydrate --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY . .
RUN cargo leptos build -r

FROM debian:bullseye-slim AS runtime
COPY --from=builder /usr/src/web/target/site site
COPY --from=builder /usr/src/web/target/server/release/web web
ENV LEPTOS_OUTPUT_NAME="web"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_RELOAD_PORT="3001"
CMD ["./web"]
