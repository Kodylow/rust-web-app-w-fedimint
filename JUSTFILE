server:
    cargo watch -q -c -w src/ -w .cargo/ -x "run"

dev:
    cargo watch -q -c -w examples/ -w .cargo/ -x "run --example quick_dev"