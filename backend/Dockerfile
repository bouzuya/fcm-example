FROM rust:1.83.0-bookworm AS chef
RUN apt update && apt install -y musl-dev musl-tools
WORKDIR /app
RUN cargo install cargo-chef --locked
RUN rustup target add x86_64-unknown-linux-musl

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --release --target=x86_64-unknown-linux-musl

FROM chef AS builder
COPY . .
COPY --from=cacher /app/target target
RUN CARGO_BUILD_INCREMENTAL=true cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
ENV PORT=3000
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /usr/local/bin/backend
COPY --from=builder /app/public /usr/local/bin/public
WORKDIR /usr/local/bin
ENTRYPOINT ["/usr/local/bin/backend"]

