FROM rust:buster AS base

ENV USER=root

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

FROM base AS development

EXPOSE 5036

CMD [ "cargo", "run" ]

FROM base AS dev-envs

EXPOSE 5036
RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF

RUN <<EOF
useradd -s /bin/bash -m vscode
groupadd docker
usermod -aG docker vscode
EOF
# install Docker tools (cli, buildx, compose)
COPY --from=gloursdocker/docker / /
CMD [ "cargo", "run" ]

FROM base AS builder

RUN cargo build --release

FROM debian:buster-slim

RUN <<EOF
apt-get udpate
apt-get install -y sqlite3 lisqlite3-dev
EOF

EXPOSE 5036

COPY --from=builder /code/target/release/graphql-backend /graphql-backend

CMD [ "/graphql-backend" ]