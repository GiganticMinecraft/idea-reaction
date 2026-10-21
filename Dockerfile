FROM rust:1.99.0-trixie as Builder

WORKDIR /root/app
COPY --chown=root:root . .

RUN cargo build --release --bin idea-reaction

FROM debian:trixie-slim as Runner

COPY --from=Builder --chown=root:root /root/app/target/release/idea-reaction /usr/local/bin/idea-reaction

RUN apt-get update && apt-get install -y --no-install-recommends libssl3t64 ca-certificates && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --user-group idea-reaction
USER idea-reaction
WORKDIR /home/idea-reaction

LABEL org.opencontainers.image.source=https://github.com/GiganticMinecraft/idea-reaction

ENTRYPOINT [ "idea-reaction" ]
