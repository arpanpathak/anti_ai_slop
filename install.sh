#!/bin/sh
# Link every skill in this repository into ~/.claude/skills.
set -eu

source_dir=$(cd "$(dirname "$0")/skills" && pwd)
target_dir="$HOME/.claude/skills"

mkdir -p "$target_dir"

for skill in "$source_dir"/*/; do
    name=$(basename "$skill")

    if [ -d "$target_dir/$name" ] && [ ! -L "$target_dir/$name" ]; then
        echo "skipped $name: $target_dir/$name is a real directory"
        continue
    fi

    ln -sfn "${skill%/}" "$target_dir/$name"
    echo "linked $name"
done
