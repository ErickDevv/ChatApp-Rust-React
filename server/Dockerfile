FROM rust:1.71.0

WORKDIR /app

COPY . .

RUN cargo build

EXPOSE 5800

CMD ["cargo", "run"]