FROM rust AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/hasmariahcareybreachedcontainment /usr/bin/hasmariahcareybreachedcontainment
CMD ["hasmariahcareybreachedcontainment"]
