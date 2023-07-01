FROM rust:buster AS base-for-develop
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

FROM base-for-develop AS builder
RUN mkdir -p /build
ENV CARGO_TARGET_DIR=/build
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /build/release/graphql-backend /graphql-backend
EXPOSE 5036
CMD [ "/graphql-backend" ]