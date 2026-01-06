# wc

> Word, line, character, and byte count.
> See also: awk, grep -c
> Keywords: count, lines, words, characters, bytes, size

- Count lines, words, and bytes:

`wc file.txt`

- Count lines only:

`wc -l file.txt`

- Count words only:

`wc -w file.txt`

- Count characters:

`wc -m file.txt`

- Count bytes:

`wc -c file.txt`

- Count lines in multiple files:

`wc -l *.txt`

- Count lines from stdin:

`cat file.txt | wc -l`

- Count files in directory:

`ls | wc -l`

- Count lines matching pattern:

`grep "pattern" file.txt | wc -l`

- Get just the number (no filename):

`wc -l < file.txt`

- Get longest line length:

`wc -L file.txt`

## Flags

- `-l, --lines`: Print line count
- `-w, --words`: Print word count
- `-m, --chars`: Print character count
- `-c, --bytes`: Print byte count
- `-L, --max-line-length`: Print longest line length

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- Count off by one - Check if file has trailing newline
- Large byte count for character count - Use -m not -c for UTF-8
- Extra output - Filename included, use < redirect to suppress
