FROM rust:1.65.0 as build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=build /app/target/release/gli .
CMD ["./gli"]
