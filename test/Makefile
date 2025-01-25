.PHONY: up test clean

up:
	docker compose up -d --build

test:
	docker compose exec -T app cargo test --verbose -- --nocapture -- --test-threads=1

clean:
	docker compose down -v