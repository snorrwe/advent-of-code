_default:
    @just --list

_new-rs out:
    cookiecutter $PWD/template-rs -o {{ out }}

[no-cd]
new-rs:
    #!/usr/bin/env bash
    set -euo pipefail
    just _new-rs $PWD

# count lines of code in the solutions folders
loc:
    fd '^20\d{2}$|^rust-utils$' -d 1 -td -X tokei -s lines -e '*.txt' -e '*.toml'
