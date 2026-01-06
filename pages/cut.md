# cut

> Remove sections from each line of files.
> See also: awk, paste, tr
> Keywords: columns, fields, extract, delimiter, split, csv

- Extract specific field (column) from delimiter-separated data:

`cut -d',' -f2 file.csv`

- Extract multiple fields:

`cut -d':' -f1,3 /etc/passwd`

- Extract range of fields:

`cut -d',' -f1-3 file.csv`

- Extract from field N to end:

`cut -d',' -f3- file.csv`

- Extract characters by position:

`cut -c1-10 file.txt`

- Extract first 5 characters of each line:

`cut -c1-5 file.txt`

- Extract using tab delimiter (default):

`cut -f1 file.tsv`

- Extract bytes:

`cut -b1-10 file.txt`

- Use output delimiter different from input:

`cut -d',' -f1,2 --output-delimiter=' ' file.csv`

- Complement selection (everything except specified):

`cut -d',' --complement -f2 file.csv`

## Flags

- `-d, --delimiter`: Use custom field delimiter (default: TAB)
- `-f, --fields`: Select fields (columns)
- `-c, --characters`: Select characters
- `-b, --bytes`: Select bytes
- `--complement`: Complement the selection
- `--output-delimiter`: Use different output delimiter
- `-s, --only-delimited`: Don't print lines without delimiter

## Field Selection

- `-f1`: First field
- `-f1,3`: Fields 1 and 3
- `-f1-3`: Fields 1 through 3
- `-f3-`: Field 3 to end
- `-f-3`: Start to field 3

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- "delimiter must be a single character" - Use only one char for -d
- Wrong column - Fields are 1-indexed, not 0-indexed
- Whitespace not splitting - Default is TAB, use -d' ' for space
- For complex parsing, use awk instead
