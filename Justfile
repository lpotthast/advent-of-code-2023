default:
    just --list

dev:
    cargo watch -x "run --release" --why

test:
    cargo test

bench:
    cargo bench

flamegraph:
    cargo flamegraph --profile flamegraph
