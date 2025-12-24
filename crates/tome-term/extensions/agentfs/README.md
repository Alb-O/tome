# AgentFS Extension for Tome

This extension provides an arbitrary filesystem adapter for Tome, allowing the editor to operate on virtualized or remote filesystems backed by [AgentFS](https://github.com/tursodatabase/agentfs).

## Overview

Tome uses an asynchronous `FileSystem` abstraction (based on the `agentfs-sdk` traits). This allows the editor to switch its backend storage at runtime without affecting core editing logic.

The AgentFS extension manages connections to SQLite-backed virtual filesystems, which can be used for:

- Persistent per-agent workspaces.
- Ephemeral in-memory scratchpads.
- Virtualized overlays on top of the host filesystem.

## Commands

- `:agent.connect <id_or_path>`: Connect to an AgentFS instance.
  - Providing an ID (e.g., `my-workspace`) connects to `.agentfs/my-workspace.db`.
  - Providing `:memory:` creates an ephemeral in-memory filesystem.
  - Providing a path to a `.db` file connects to it directly.
- `:agent.disconnect`: Reverts to the host filesystem.

## Technical Implementation

### Filesystem API

The editor core no longer uses `std::fs`. Instead, it holds an `Arc<dyn FileSystem>`.
All I/O operations in the editor (`save`, `load`, etc.) are `async` and non-blocking, ensuring the UI remains responsive even during high-latency database operations.

### Git Subtree

The `agentfs-sdk` is managed via a git subtree. See [vendor/README.md](../../../../vendor/README.md) for instructions on how to sync with upstream or contribute local patches back.
