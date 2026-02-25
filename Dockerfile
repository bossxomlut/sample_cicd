FROM debian:bullseye-slim
WORKDIR /app
COPY target/release/rust_rest_api ./rust_rest_api
EXPOSE 8080
CMD ["./rust_rest_api"]
