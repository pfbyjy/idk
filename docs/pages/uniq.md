# uniq

> Report or filter out repeated lines. INPUT MUST BE SORTED FIRST.
> See also: sort, comm, wc
> Keywords: unique, duplicate, repeated, lines, count, dedupe

- Remove adjacent duplicate lines (MUST be sorted first):

`sort file.txt | uniq`

- Count occurrences of each line:

`sort file.txt | uniq -c`

- Show only duplicate lines:

`sort file.txt | uniq -d`

- Show only unique lines (appear once):

`sort file.txt | uniq -u`

- Ignore case when comparing:

`sort file.txt | uniq -i`

- Skip first N fields when comparing:

`sort file.txt | uniq -f 1`

- Skip first N characters when comparing:

`sort file.txt | uniq -s 5`

- Compare only first N characters:

`sort file.txt | uniq -w 10`

- Count and sort by frequency:

`sort file.txt | uniq -c | sort -rn`

- Remove duplicates from unsorted file (using sort):

`sort -u file.txt`

## Flags

- `-c, --count`: Prefix lines with number of occurrences
- `-d, --repeated`: Only print duplicate lines
- `-u, --unique`: Only print unique lines (that appear once)
- `-i, --ignore-case`: Ignore differences in case
- `-f, --skip-fields`: Skip first N fields
- `-s, --skip-chars`: Skip first N characters
- `-w, --check-chars`: Compare only first N characters

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- Not removing all duplicates - INPUT MUST BE SORTED FIRST
- Using on unsorted data - uniq only removes adjacent duplicates
- Use sort -u if you just need unique lines from unsorted data
