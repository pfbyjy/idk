# du

> Estimate file and directory space usage.
> See also: df, ncdu, ls -l
> Keywords: disk, usage, size, directory, space, storage

- Show size of a directory:

`du -sh directory/`

- Show size of current directory:

`du -sh .`

- Show size of all subdirectories:

`du -h --max-depth=1`

- Show size of each item in directory:

`du -sh *`

- Show and sort by size:

`du -sh * | sort -h`

- Find largest directories:

`du -h --max-depth=1 | sort -hr | head -10`

- Show apparent size (not disk usage):

`du -sh --apparent-size directory/`

- Exclude certain patterns:

`du -sh --exclude='*.log' directory/`

- Show total only:

`du -s directory/`

- Follow symbolic links:

`du -shL directory/`

- Show size in specific units:

`du -sk directory/`

- Find large files:

`du -ah . | sort -hr | head -20`

## Flags

- `-h, --human-readable`: Human-readable sizes
- `-s, --summarize`: Display only total for each argument
- `-c, --total`: Produce a grand total
- `-a, --all`: Show all files, not just directories
- `-d, --max-depth`: Max directory depth to show
- `--exclude`: Skip files matching pattern
- `-L, --dereference`: Follow symbolic links
- `--apparent-size`: Print apparent sizes, not disk usage
- `-k`: Show in kilobytes
- `-m`: Show in megabytes

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- "Permission denied" - Need read access to directories
- Slow on large directories - Use ncdu for interactive exploration
- Size differs from ls - du shows disk blocks, ls shows file size
