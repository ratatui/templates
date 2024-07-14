default:
    @just --list

generate-all:
    just generate-component
    just generate-simple
    just generate-simple-async

generate-component:
    rm -rf component-generated
    cargo generate --path ./component \
        --name component-generated \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false

generate-simple:
    rm -rf simple-generated
    cargo generate --path ./simple --name simple-generated

generate-simple-async:
    rm -rf simple-async-generated
    cargo generate --path ./simple-async --name simple-async-generated
