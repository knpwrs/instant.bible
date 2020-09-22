# Build image
FROM rust:1.46-buster as build

WORKDIR /app
COPY . .

ENV SERVER_SENTRY_DSN=https://9965738e8eac44b9922b71951141f59d@o387306.ingest.sentry.io/5222373

RUN cargo build --release --bin server

# Final deployment image
FROM rust:1.46-slim-buster

COPY --from=build /app/target/release/server .
COPY --from=build /app/index.pb .

ENV RUST_LOG=engine=info,server=info

EXPOSE 8081

CMD ./server
