SERVICE = rustydo

run: build
	docker compose run -it --rm $(SERVICE)

build:
	docker compose run --rm $(SERVICE) cargo build --release
