FROM rust

RUN mkdir -p app

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

CMD ["cargo-watch", "-x", "'run -- -release'"]