FROM rust:1.65.0 as build
RUN update-ca-certificates
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=build /app/target/release/gli .
CMD ["./gli"]
