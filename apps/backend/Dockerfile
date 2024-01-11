FROM rust:bookworm AS builder

WORKDIR /backend
COPY . .
RUN cargo build --release

# production stage
FROM debian:bookworm-slim AS runner
 
WORKDIR /backend
COPY --from=builder /backend/target/release/infrastructure /bin/
ENV RUST_ENV=production
RUN apt-get update && \
    apt-get install -y --no-install-recommends openssl libssl-dev
EXPOSE 8000
CMD ["/bin/infrastructure"]