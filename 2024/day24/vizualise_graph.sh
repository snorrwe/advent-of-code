#!/usr/bin/env nix-shell
#! nix-shell -i bash --pure
#! nix-shell -p bash graphviz

dot -O -Tsvg graph
