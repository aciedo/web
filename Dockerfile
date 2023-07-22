FROM ghcr.io/rust-lang/rust:nightly as build
LABEL org.opencontainers.image.source https://github.com/valeralabs/web
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo install cargo-leptos
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/web

COPY . .

RUN cargo leptos build -r

FROM ghcr.io/rust-lang/rust:nightly-slim

COPY --from=build /usr/src/web/target/site site
COPY --from=build /usr/src/web/target/server/release/web web

ENV LEPTOS_OUTPUT_NAME="web"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_RELOAD_PORT="3001"

CMD ["./web"]
