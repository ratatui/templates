default:
    @just --list

generate-all:
    just generate-component
    just generate-hello-world
    just generate-simple
    just generate-simple-async
    just generate-event-driven
    just generate-event-driven-async

# The purpose of these targets is to make it easy to make changes to the templates and then
# regenerate the generated projects and view the expected changes in a git diff.

generate-component $CARGO_NAME="your name" $CARGO_EMAIL="author@example.com":
    rm -rv component-generated
    cargo generate --path ./component \
        --name component-generated \
        --define project-description="An example generated using the component template" \
        --define use-gitserver=false

generate-hello-world $CARGO_NAME="your name" $CARGO_EMAIL="author@example.com":
    rm -rv hello-world-generated
    cargo generate --path ./hello-world --name hello-world-generated

generate-simple $CARGO_NAME="your name" $CARGO_EMAIL="author@example.com":
    rm -rv simple-generated
    cargo generate --path ./simple --name simple-generated

generate-simple-async $CARGO_NAME="your name" $CARGO_EMAIL="author@example.com":
    rm -rv simple-async-generated
    cargo generate --path ./simple-async --name simple-async-generated

generate-event-driven $CARGO_NAME="your name" $CARGO_EMAIL="author@example.com":
    rm -rv event-driven-generated
    cargo generate --path ./event-driven --name event-driven-generated

generate-event-driven-async $CARGO_NAME="your name" $CARGO_EMAIL="author@example.com":
    rm -rv event-driven-async-generated
    cargo generate --path ./event-driven-async --name event-driven-async-generated
