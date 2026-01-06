# head

> Output the first part of files.
> See also: tail, cat, less
> Keywords: first, lines, beginning, top, preview

- Show first 10 lines of a file (default):

`head file.txt`

- Show first N lines:

`head -n 20 file.txt`

- Show first N bytes:

`head -c 100 file.txt`

- Show all but last N lines:

`head -n -5 file.txt`

- Show first lines of multiple files:

`head file1.txt file2.txt`

- Show first line of multiple files (useful for checking):

`head -n 1 *.csv`

- Pipe usage - show first 10 results:

`grep "error" log.txt | head`

- Show first 10 lines quietly (no filename header):

`head -q file1.txt file2.txt`

## Flags

- `-n, --lines`: Output first N lines (or -N for all but last N)
- `-c, --bytes`: Output first N bytes
- `-q, --quiet`: Never print headers with file names
- `-v, --verbose`: Always print headers with file names

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Common Errors

- "cannot open for reading" - File doesn't exist or no permission
