FROM rust:1.84

RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /app
COPY . .

CMD ["tail", "-f", "/dev/null"]