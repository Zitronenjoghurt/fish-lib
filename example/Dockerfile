FROM rust:1.84

RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /app
COPY . .

WORKDIR /app/example
CMD ["tail", "-f", "/dev/null"]

#RUN cargo build
#CMD ["./target/debug/example"]