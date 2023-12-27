FROM rust:1 AS build

COPY Cargo.toml Cargo.lock /app/
# Have to do this so cargo wont complain
WORKDIR /app
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY src src/
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /app/target/release/football-reminder-bot /usr/local/bin/football-reminder-bot
CMD ["/usr/local/bin/football-reminder-bot"]
