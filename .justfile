set shell := ["fish", "-c"]

_default: _list

_list:
    just --list

public-api:
    cargo public-api

coverage:
    cargo tarpaulin

docs:
    cargo clean; cargo doc --lib --open --no-deps

# update-gif:
#     cargo install --path .
#     vhs scripts/demo.tape -o docs/out.gif

publish:
    #!/usr/bin/env fish
    set CURRENT_BRANCH (git symbolic-ref --short HEAD)
    if [ $CURRENT_BRANCH != main ]
        echo "Not on master branch. Please switch to master before publishing."
        exit 1
    end
    set NEXT_VERSION (just _next-version)
    gum confirm "Confirm next version: '$NEXT_VERSION'?"; or exit 1
    just _check-repo; or exit 1
    cargo clippy --fix
    just _check-repo; or exit 1
    cargo test; or exit 1
    just update-usage
    echo "Updating Cargo.toml to version $NEXT_VERSION"
    toml set Cargo.toml package.version $NEXT_VERSION| sponge Cargo.toml
    gum confirm "Update gif?"; and just update-gif; git add docs/out.gif
    gum confirm "git commit -a"; and git commit -a
    gum confirm "git tag?"; and git tag $NEXT_VERSION
    gum confirm "git push?"; and git push --tags origin main
    gum confirm "Rust publish"; and cargo publish

_check-repo:
    #!/usr/bin/env fish
    set is_dirty (git status --porcelain)
    if test -n "$is_dirty"
        echo "Repo is dirty. Please commit all changes before publishing."
        exit 1
    end

_next-version:
    #!/usr/bin/env fish
    set LATEST_TAG (git describe --tags --abbrev=0)
    set PARTS (string split . $LATEST_TAG)
    set MAJOR $PARTS[1]
    set MINOR $PARTS[2]
    set PATCH $PARTS[3]
    set NEXT_PATCH (math $PATCH + 1)
    echo "$MAJOR.$MINOR.$NEXT_PATCH"

# bump-version:
#     toml set Cargo.toml package.version XXXXX | sponge Cargo.toml

cargo-analytics:
    cargo tree > docs/tree.txt
    cargo bloat --release --crates -n 10000 > docs/bloat.txt
    cargo report future-incompatibilities > docs/future-incompatibilities.txt; or true
    unused-features analyze
    unused-features build-report --input report.json
    mv report.json docs/unused-features.json
    mv report.html docs/unused-features.html
    #unused-features prune --input docs/unused-features.json

cargo-installs:
    cargo install cargo-bloat
    brew install cargo-udeps
    cargo install cargo-unused-features
    cargo install toml-cli
    #rustup toolchain install nightly-aarch64-apple-darwin
