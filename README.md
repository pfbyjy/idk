# idk

man pages for agents - semantic search for commands

## What is this?

`idk` is like `tldr`, but designed for AI agents:

- **Semantic search**: Don't know the command? Search by description
- **Structured output**: JSON mode for machine parsing
- **Agent-friendly format**: Examples first, exit codes, common errors
- **Remote sync**: Fetch pages from a central repo, cache locally

## Usage

```bash
# Show help for a known command
idk curl

# Search for commands by description
idk "make http request"
idk "find files by name"

# JSON output for machine parsing
idk --json curl

# List available pages
idk --list

# Sync pages from remote
idk --sync

# Force update from remote
idk --update curl
```

## Installation

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
./target/release/idk
```

## Page Format

Pages are markdown files with a simple structure:

```markdown
# command

> Short description of what the command does.
> See also: related, commands
> Keywords: search, terms, for, semantic, matching

- Description of example:

`command --flag argument`

- Another example:

`command other-usage`

## Flags

- `-f, --flag`: What this flag does

## Exit Codes

- `0`: Success
- `1`: Error description

## Common Errors

- Error message - what it means and how to fix
```

## Configuration

Set `IDK_REMOTE` to use a custom pages repository:

```bash
export IDK_REMOTE="https://raw.githubusercontent.com/your/repo/main/pages"
```

## Page Locations

Pages are searched in this order:

1. `pages/` directory next to the binary
2. `pages/` in current working directory
3. `~/.cache/idk/pages/` (downloaded from remote)
