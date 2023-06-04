FROM rust:1.69 as builder
RUN apt-get update && apt-get install -y curl
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
EXPOSE 8080
COPY --from=builder /usr/bin/curl /usr/local/bin/curl
COPY --from=builder /usr/local/cargo/bin/rust_ddd_example /usr/local/bin/rust_ddd_example
CMD ["rust_ddd_example"]