ARG RUST_VERSION=1.74.0

FROM rust:${RUST_VERSION}-slim-bookworm as builder

# tailwind
WORKDIR /usr/bin/
ADD https://github.com/tailwindlabs/tailwindcss/releases/download/v3.3.5/tailwindcss-linux-x64 tailwindcss
RUN chmod +x /usr/bin/tailwindcss

# openssl + CA certs
RUN apt-get update; \
    apt-get install -y --no-install-recommends ca-certificates pkg-config libssl-dev

WORKDIR /usr/src/plcom
COPY css/ css/
COPY public/ public/
COPY templates/ templates/
COPY src/ src/
COPY build.rs .
COPY Cargo.lock .
COPY Cargo.toml .
COPY tailwind.config.cjs .

# generate wallpapers
RUN cargo build --bin gen-wallpapers --release
RUN cargo run --bin gen-wallpapers --release

# build project
RUN cargo build --bin plcom --release

FROM debian:12-slim
WORKDIR /usr/share/plcom
COPY --from=builder /usr/src/plcom/public /usr/share/plcom/public
COPY --from=builder /usr/src/plcom/target/release/plcom /usr/share/plcom

ENV ROCKET_CLI_COLORS=0
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
ENTRYPOINT ["/usr/share/plcom/plcom"]
