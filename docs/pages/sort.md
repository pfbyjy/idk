# sort

> Sort lines of text files.
> See also: uniq, shuf, tsort
> Keywords: sort, order, alphabetical, numerical, lines

- Sort file alphabetically:

`sort file.txt`

- Sort in reverse order:

`sort -r file.txt`

- Sort numerically:

`sort -n file.txt`

- Sort by human-readable numbers (1K, 2M, 3G):

`sort -h file.txt`

- Sort and remove duplicates:

`sort -u file.txt`

- Sort by specific column (field):

`sort -k2 file.txt`

- Sort by second column numerically:

`sort -k2 -n file.txt`

- Sort by multiple columns:

`sort -k1,1 -k2,2n file.txt`

- Sort with custom delimiter:

`sort -t',' -k2 file.csv`

- Sort case-insensitive:

`sort -f file.txt`

- Sort by month name:

`sort -M file.txt`

- Check if already sorted:

`sort -c file.txt`

- Sort in place (using sponge or temp file):

`sort file.txt -o file.txt`

- Stable sort (preserve order of equal elements):

`sort -s -k1 file.txt`

- Random sort (shuffle):

`sort -R file.txt`

## Flags

- `-r, --reverse`: Reverse the result
- `-n, --numeric-sort`: Compare according to numerical value
- `-h, --human-numeric-sort`: Compare human readable numbers (2K, 1G)
- `-k, --key`: Sort by specific field
- `-t, --field-separator`: Use custom field separator
- `-u, --unique`: Output only unique lines
- `-f, --ignore-case`: Case-insensitive
- `-c, --check`: Check if input is sorted
- `-o, --output`: Write result to file
- `-s, --stable`: Stabilize sort by disabling last-resort comparison
- `-R, --random-sort`: Shuffle (random order)
- `-M, --month-sort`: Compare month names

## Exit Codes

- `0`: Success (or sorted if -c)
- `1`: With -c, input is not sorted
- `2`: Error

## Common Errors

- "write failed" - Disk full or permission issue
- Wrong sort order - Use -n for numbers, -V for versions
- Column mismatch - Check -t delimiter and -k column number
