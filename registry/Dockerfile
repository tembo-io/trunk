FROM quay.io/tembo/rust:1.76-bookworm as builder
COPY .sqlx  .sqlx
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations/
RUN cargo build --release && \
    cargo clean -p trunk-registry
RUN cargo install --path .

# second stage.
FROM quay.io/tembo/debian:12.5-slim
COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin/
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y --no-install-recommends ca-certificates curl unzip
RUN update-ca-certificates

# install AWS CLI
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
    unzip awscliv2.zip && \
    ./aws/install