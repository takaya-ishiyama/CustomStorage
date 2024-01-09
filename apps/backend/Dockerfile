FROM rust:latest

# WORKDIR /apps/backend
# COPY . /apps/backend/
# EXPOSE 8000
# CMD ["cargo", "run"]

# production stage
FROM debian:bookworm-slim AS runner
 
WORKDIR /apps/backend
COPY . /apps/backend/
CMD ["cargo", "run"]