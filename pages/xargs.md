# xargs

> Build and execute commands from standard input.
> See also: parallel, find -exec
> Keywords: pipe, arguments, batch, command, parallel, stdin

- Basic usage (pass stdin as arguments):

`echo "file1 file2" | xargs rm`

- Use with find:

`find . -name "*.txt" | xargs cat`

- Handle filenames with spaces (null delimiter):

`find . -name "*.txt" -print0 | xargs -0 rm`

- One argument per command:

`cat urls.txt | xargs -n1 curl`

- Limit arguments per command:

`echo "1 2 3 4 5 6" | xargs -n2 echo`

- Replace string placeholder:

`cat files.txt | xargs -I {} cp {} /backup/`

- Parallel execution:

`cat urls.txt | xargs -P4 -n1 curl`

- Prompt before each command:

`find . -name "*.tmp" | xargs -p rm`

- Show command being executed:

`cat files.txt | xargs -t cp -t /backup/`

- Max command line length:

`cat files.txt | xargs -s 1000 echo`

- Handle empty input gracefully:

`echo "" | xargs --no-run-if-empty rm`

- Append to specific position:

`find . -name "*.jpg" | xargs -I {} convert {} -resize 50% {}`

## Flags

- `-0, --null`: Input items are null-terminated
- `-n N`: Use at most N arguments per command
- `-I {}`: Replace {} with input (implies -n1)
- `-P N`: Run N processes in parallel
- `-p, --interactive`: Prompt before each command
- `-t, --verbose`: Print commands before executing
- `-r, --no-run-if-empty`: Don't run if input is empty
- `-s N`: Max command line length
- `-L N`: Use at most N lines per command

## Exit Codes

- `0`: Success
- `123`: Any invocation exited with 1-125
- `124`: Command exited with 255
- `125`: Command killed by signal
- `126`: Command cannot be run
- `127`: Command not found
- `1`: Other error

## Common Patterns

```bash
# Process files with spaces
find . -print0 | xargs -0

# Parallel downloads
cat urls.txt | xargs -P8 -n1 wget

# Delete old files
find . -mtime +30 | xargs rm

# Replace in files
find . -name "*.txt" | xargs sed -i 's/old/new/g'
```

## Common Errors

- "Argument list too long" - Use xargs to batch
- Breaks on spaces in filenames - Use -0 with find -print0
- Command runs with no args - Use --no-run-if-empty
