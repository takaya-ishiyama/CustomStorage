FROM rust:bookworm AS builder

WORKDIR /backend
COPY . .
RUN cargo build --release

# # production stage
FROM debian:bookworm-slim AS runner
 
WORKDIR /backend
COPY --from=builder /backend/target/release/infrastructure /bin/
# EXPOSE 8001
CMD ["/bin/infrastructure"]