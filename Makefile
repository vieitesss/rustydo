# The Docker image that will be created after building
IMAGE = rustydo
# The source files of the project
SOURCES = $(wildcard src/*.rs src/**/*.rs src/bin/*.rs)
# Hidden file needed in order to redo the building process only if the source
# files have been changed
TIMESTAMP = .last_build

# Default command (`make`)
all: run

# Rebuild the docker image and update the timestamp if the source files have
# changed
$(TIMESTAMP): $(SOURCES)
	@docker rmi -f $(IMAGE) || true
	@docker build -t $(IMAGE) .
	touch $(TIMESTAMP)

# Run the Docker container using the image previously built and remove the
# container after exit
run: $(TIMESTAMP)
	@docker run -it --rm $(IMAGE)

# Only build the Docker image, without executing the whole app
build: $(TIMESTAMP)

.PHONY: all run build
