FROM rust:1.90.0-bullseye as Builder

WORKDIR /root/app
COPY --chown=root:root . .

RUN cargo build --release --bin idea-reaction

FROM debian:bullseye-slim as Runner

COPY --from=Builder --chown=root:root /root/app/target/release/idea-reaction /usr/local/bin/idea-reaction

RUN apt-get update && apt-get install -y libssl-dev ca-certificates

RUN useradd --create-home --user-group idea-reaction
USER idea-reaction
WORKDIR /home/idea-reaction

LABEL org.opencontainers.image.source=https://github.com/GiganticMinecraft/idea-reaction

ENTRYPOINT [ "idea-reaction" ]
