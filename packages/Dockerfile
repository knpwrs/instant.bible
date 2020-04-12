FROM rust:1.42-slim

WORKDIR /app
COPY . .

RUN cargo build --release --bin server

EXPOSE 8081

ENV RUST_LOG=engine=info,server=info,actix_web=info
CMD ./target/release/server
