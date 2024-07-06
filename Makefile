IMAGE = rustydo

run: build
	@docker run -it -it --rm $(IMAGE)

build: Dockerfile src/bin/main.rs Cargo.toml
	@docker rmi -f $(IMAGE) || true
	@docker build -t $(IMAGE) .

.PHONY: build run
