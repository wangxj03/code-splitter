docs:
    cargo doc --open --no-deps --all-features

integration:
    cargo test --test markdown
