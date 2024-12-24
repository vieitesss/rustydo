alias r := run

default:
  just -l

run:
  cargo run --release
