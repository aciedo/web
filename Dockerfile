FROM ghcr.io/rust-lang/rust:nightly as build
ENV PKG_CONFIG_ALLOW_CROSS=1
LABEL org.opencontainers.image.source https://github.com/valeralabs/web

WORKDIR /usr/src/web
COPY . .

RUN cargo install cargo-leptos
RUN rustup target add wasm32-unknown-unknown
RUN cargo leptos build -r
RUN cargo install --path . --no-default-features --features ssr

FROM gcr.io/distroless/cc-debian10

# copy the server
COPY --from=build /usr/local/cargo/bin/valera_web /usr/local/bin/valera_web
# copy static outputs
COPY --from=build target/site target/site

ENV LEPTOS_OUTPUT_NAME=valera_web

CMD ["valera_web"]
