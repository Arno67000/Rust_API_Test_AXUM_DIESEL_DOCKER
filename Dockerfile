# Build stage
FROM rust:1-buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Production stage
FROM debian:bullseye-slim as runner

WORKDIR /usr/local/bin

RUN apt-get update && apt-get install libpq-dev -y

COPY --from=builder /app/target/release/api-test .

CMD [ "./api-test" ]