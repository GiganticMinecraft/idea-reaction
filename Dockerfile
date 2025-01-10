FROM rust:1.83.0-bookworm AS builder

WORKDIR /root/app
COPY --chown=root:root . .

RUN cargo build --release --bin idea-reaction

FROM gcr.io/distroless/cc-debian12 AS runner

COPY --from=Builder --chown=root:root /root/app/target/release/idea-reaction /

LABEL org.opencontainers.image.source=https://github.com/GiganticMinecraft/idea-reaction

ENTRYPOINT ["/idea-reaction"]
