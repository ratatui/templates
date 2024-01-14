default:
  @just --list

fmt:
    cargo fmt
    prettier --write .
    just --fmt --unstable

update:
    cargo upgrade --incompatible
    cargo update

check:
    pre-commit run --all-files
    cargo check
    cargo clippy

build:
    cargo build --all-targets

test:
    cargo test run --workspace --all-targets

changelog:
    git cliff -o CHANGELOG.md
    prettier --write CHANGELOG.md

binary-name := "ratatui-hello-world"

generate-hello-world:
    cargo generate --path . --name {{binary-name}} -d project-description="Hello World project using ratatui-template" -d gh-username=kdheepak -d msrv="stable"

generate:
    @just clean
    @just generate-hello-world

clean:
    rm -rf {{binary-name}}

generate-and-run:
    @just generate
    cd {{binary-name}} && cargo run
    @just clean

