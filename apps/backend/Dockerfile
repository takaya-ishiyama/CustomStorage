FROM rust:latest

WORKDIR /apps/backend
COPY . /apps/backend/


# Final run stage
FROM debian:bookworm-slim AS runner
 
WORKDIR /apps/backend
COPY . /apps/backend/
CMD ["cargo", "run"]