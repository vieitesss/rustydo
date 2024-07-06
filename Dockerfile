FROM rust:alpine as build

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo fetch

COPY ./src ./src

RUN cargo build --release

FROM alpine:3.20

RUN apk add --no-cache libgcc libstdc++ musl libc6-compat

WORKDIR /app

COPY --from=build /app/target/release/rustydo .

CMD ["./rustydo"]
