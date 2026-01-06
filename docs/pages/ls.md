# ls

> List directory contents. The most basic file exploration command.
> See also: tree, find, dir
> Keywords: list, files, directory, folder, contents, show

- List files in current directory:

`ls`

- List all files including hidden (dotfiles):

`ls -a`

- List with detailed info (permissions, size, date):

`ls -l`

- List all files with details (combine -l and -a):

`ls -la`

- List with human-readable file sizes:

`ls -lh`

- List sorted by modification time (newest first):

`ls -lt`

- List sorted by size (largest first):

`ls -lS`

- List only directories:

`ls -d */`

- List files matching a pattern:

`ls *.txt`

- List with file type indicators (/ for dirs, * for executables):

`ls -F`

- List recursively:

`ls -R`

- List one file per line:

`ls -1`

## Flags

- `-a, --all`: Show hidden files (starting with .)
- `-l`: Long format with details
- `-h, --human-readable`: Human-readable sizes (use with -l)
- `-t`: Sort by modification time
- `-S`: Sort by file size
- `-r, --reverse`: Reverse sort order
- `-R, --recursive`: List subdirectories recursively
- `-d, --directory`: List directories themselves, not contents
- `-1`: One file per line
- `-F, --classify`: Append indicator (*/=>@|) to entries

## Exit Codes

- `0`: Success
- `1`: Minor problem (e.g., cannot access subdirectory)
- `2`: Serious trouble (e.g., cannot access command-line argument)

## Common Errors

- "No such file or directory" - Path doesn't exist, check spelling
- "Permission denied" - Need read permission on directory
