FROM rust:bookworm AS builder

WORKDIR /backend
COPY . .
RUN cargo build --release

# production stage
FROM debian:bookworm-slim AS runner
 
WORKDIR /backend
COPY --from=builder /backend/target/release/infrastructure /bin/
RUN apt-get update && \
    apt-get install -y --no-install-recommends openssl libssl-dev
# CMD ["/usr/bin/env", "-i", "RUST_ENV=production", "/bin/infrastructure"]
# EXPOSE 8001
CMD ["/bin/infrastructure"]