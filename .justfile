public-api:
    cargo public-api

coverage:
    cargo tarpaulin

docs:
    cargo clean; cargo doc --lib --open --no-deps
