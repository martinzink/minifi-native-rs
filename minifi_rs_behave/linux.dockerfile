FROM rust:slim AS chef
RUN apt-get update && apt-get install -y clang lld pkg-config
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

# 5. Export Stage
FROM scratch AS bin-export
COPY --from=builder /app/target/release/libminifi_rs_playground.so /
COPY --from=builder /app/target/release/libminifi_pgp.so /