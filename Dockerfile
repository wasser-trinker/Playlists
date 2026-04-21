FROM rust:1.95

WORKDIR /app/
COPY . .
CMD ["cargo", "run", "--release"]