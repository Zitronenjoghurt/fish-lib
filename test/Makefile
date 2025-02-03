.PHONY: up test test-backtrace clean

up:
	docker compose up -d --build

test:
	docker compose exec -T app cargo test -- --nocapture --test-threads=1

test-backtrace:
	docker compose exec -T -e RUST_BACKTRACE=1 app cargo test -- --nocapture --test-threads=1

clean:
	docker compose down -v
	docker image prune -f