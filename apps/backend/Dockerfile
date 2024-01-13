FROM rust:bookworm AS builder

WORKDIR /backend
COPY . .
# RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres && \
#     cargo sqlx prepare --workspace && \
#     cargo sqlx prepare --check --workspace
# RUN cd infrastructure && \
#     cargo sqlx prepare --workspace && \
#     cargo sqlx prepare --check --workspace
# ENV SQLX_OFFLINE=true
# RUN cargo build --release

# production stage
# FROM debian:bookworm-slim AS runner
 
# WORKDIR /backend
# ENV RUST_ENV=production
# ENV SQLX_OFFLINE=true
# COPY --from=builder /backend/.sqlx /backend/
# COPY --from=builder /backend/target/sqlx-prepare-check /bin/
# COPY --from=builder /backend/target/release/infrastructure /bin/
# RUN apt-get update && \
#     apt-get install -y --no-install-recommends openssl libssl-dev
EXPOSE 8000
# CMD ["/bin/infrastructure"]

#
CMD ["cargo", "run"]