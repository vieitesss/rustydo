IMAGE = rustydo

run: 
	@docker run --rm $(IMAGE)

build:
	@docker rmi -f $(IMAGE) .
	@docker build -t $(IMAGE) .

.PHONY: build run

