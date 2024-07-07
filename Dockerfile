# This is a multi-stage Dockerfile. It allows using multiple images in order to
# get a last image with the minimum weight possible

# Primary image, to compile the application
FROM rust:1.79-slim as build

# Change the working directory
WORKDIR /app

# Copy the dependency files into the image
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Download the dependencies without compiling the application
RUN cargo fetch

# Copy the source files into the image
COPY ./src ./src

# Compile the application
RUN cargo build --release

# The image from which we will run the application
FROM alpine:3.20

# Install some required dependencies to run a Rust executable
RUN apk add --no-cache libgcc libstdc++ musl libc6-compat

# Change the working directory
WORKDIR /app

# Copy the previously created executable into this image
COPY --from=build /app/target/release/rustydo .

# Run the application executable
CMD ["./rustydo"]
