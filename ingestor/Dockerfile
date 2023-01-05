FROM rust:buster AS chef

RUN apt update
RUN apt upgrade -y
RUN apt install -y cmake

RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build

FROM debian:buster-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/debug/ingestor /usr/local/bin
ENTRYPOINT ["/usr/local/bin/ingestor"]
