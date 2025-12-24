#!/usr/bin/env bash
# This file tracks external repositories integrated via git subtree.
# Usage: ./vendor/subtrees.sh [pull|push] <name> [branch]

set -e

# Repository definitions
declare -A REPOS
REPOS[agentfs]="https://github.com/tursodatabase/agentfs"

# Prefix definitions
declare -A PREFIXES
PREFIXES[agentfs]="vendor/agentfs"

COMMAND=$1
NAME=$2
BRANCH=${3:-main}

if [[ -z "$COMMAND" || -z "$NAME" ]]; then
    echo "Usage: $0 [pull|push] <name> [branch]"
    echo "Available subtrees:"
    for key in "${!REPOS[@]}"; do
        echo "  - $key (${PREFIXES[$key]})"
    done
    exit 1
fi

URL=${REPOS[$NAME]}
PREFIX=${PREFIXES[$NAME]}

if [[ -z "$URL" ]]; then
    echo "Error: Unknown subtree '$NAME'"
    exit 1
fi

echo "Executing: git subtree $COMMAND --prefix $PREFIX $URL $BRANCH --squash"
git subtree "$COMMAND" --prefix "$PREFIX" "$URL" "$BRANCH" --squash
