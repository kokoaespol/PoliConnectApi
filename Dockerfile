FROM rust:1.77-slim-buster as builder
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/PoliConnectApi /usr/local/bin/PoliConnectApi
ENV PORT=3000
EXPOSE ${PORT}
CMD ["PoliConnectApi"]
