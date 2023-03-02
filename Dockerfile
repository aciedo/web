FROM ghcr.io/rust-lang/rust:nightly as build
ENV PKG_CONFIG_ALLOW_CROSS=1
LABEL org.opencontainers.image.source https://github.com/valeralabs/web

WORKDIR /usr/src/web
RUN cargo install cargo-leptos
RUN rustup target add wasm32-unknown-unknown

COPY . .
RUN cargo leptos build -r

FROM gcr.io/distroless/cc-debian10
COPY --from=build /usr/src/web/target/server/release/valera_web /usr/local/bin/valera_web
COPY --from=build /usr/src/web/target/site target/site
ENV LEPTOS_OUTPUT_NAME=valera_web

CMD ["valera_web"]
