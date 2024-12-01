default:
    @just --list

_new-rs out:
    cookiecutter $PWD/template-rs -o {{out}}

[no-cd]
new-rs:
    #!/usr/bin/env bash
    set -euo pipefail
    just _new-rs $PWD
