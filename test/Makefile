.PHONY: up test clean

up:
	docker compose up -d --build

test:
	docker compose exec -T app cargo test

clean:
	docker compose down -v