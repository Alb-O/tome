# Vendor Directory

This directory contains external dependencies and SDKs that are integrated directly into the Tome source tree using `git subtree`.

## Management

Subtrees are tracked in `vendor/subtrees.sh`. This script provides a central place to manage upstream synchronization.

### Syncing with Upstream (Pulling)

To pull the latest changes from an upstream repository:

```bash
./vendor/subtrees.sh pull <name> [branch]
```

Example: `./vendor/subtrees.sh pull agentfs main`

### Contributing Back (Pushing)

If you have made local changes to a vendored directory and wish to contribute them back to the original repository:

```bash
./vendor/subtrees.sh push <name> [branch]
```

## Available Subtrees

| Name      | Path             | Source                                   |
| --------- | ---------------- | ---------------------------------------- |
| `agentfs` | `vendor/agentfs` | https://github.com/tursodatabase/agentfs |

## Why Subtrees?

We use subtrees instead of submodules to fulfill our "suckless" goal:

1. **Simplified Cloning**: No need for recursive clones or `--init`.
1. **Easy Patching**: You can modify vendored code directly in this repo and commit it.
1. **Atomic History**: The state of dependencies is captured in the main repository's commit history.
