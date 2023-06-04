FROM rust:1.69 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
EXPOSE 8080
COPY --from=builder /usr/local/cargo/bin/rust_ddd_example /usr/local/bin/rust_ddd_example
CMD ["rust_ddd_example"]