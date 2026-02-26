FROM scratch
WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/rust_rest_api ./rust_rest_api
COPY .env .env
EXPOSE 8080
CMD ["./rust_rest_api"]
