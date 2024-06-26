FROM rust as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin rust-pipeline

FROM rust as runtime
WORKDIR app
COPY --from=builder /app/target/release/rust-pipeline /app
RUN mkdir /tmp/bin > /dev/null
COPY --from=builder /app/target/release/rust-pipeline /tmp/bin/rust-pipeline
CMD ["/app/rust-pipeline"]