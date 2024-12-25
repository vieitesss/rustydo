alias r := run
alias b := build

default:
  just -l

run:
  cargo run --release

build:
  cargo build --release
