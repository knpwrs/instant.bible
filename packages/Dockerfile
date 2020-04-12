FROM rust:1.42-slim

WORKDIR /app
COPY . ./src

RUN cd ./src && \
  cargo build --release --bin server && \
  cd .. && \
  cp ./src/target/release/server . && \
  cp ./src/index.pb . && \
  rm -r ./src

EXPOSE 8081

ENV RUST_LOG=engine=info,server=info
CMD ./server
