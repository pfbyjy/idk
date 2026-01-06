# awk

> Pattern scanning and text processing language. Extremely powerful for columnar data.
> See also: sed, cut, perl
> Keywords: columns, fields, text, process, pattern, split, csv, data

- Print specific column (space-separated):

`awk '{print $2}' file.txt`

- Print multiple columns:

`awk '{print $1, $3}' file.txt`

- Print last column:

`awk '{print $NF}' file.txt`

- Print second-to-last column:

`awk '{print $(NF-1)}' file.txt`

- Use custom field separator:

`awk -F',' '{print $1}' file.csv`

- Use tab as separator:

`awk -F'\t' '{print $1}' file.tsv`

- Print lines matching pattern:

`awk '/pattern/' file.txt`

- Print column where another column matches:

`awk '$1 == "value" {print $2}' file.txt`

- Sum a column:

`awk '{sum += $1} END {print sum}' file.txt`

- Count lines:

`awk 'END {print NR}' file.txt`

- Print line numbers:

`awk '{print NR, $0}' file.txt`

- Print lines longer than 80 chars:

`awk 'length > 80' file.txt`

- Multiple conditions:

`awk '$1 > 100 && $2 == "active"' file.txt`

- Format output:

`awk '{printf "%-10s %5d\n", $1, $2}' file.txt`

- Skip header line:

`awk 'NR > 1 {print $1}' file.txt`

- Calculate average:

`awk '{sum += $1; count++} END {print sum/count}' file.txt`

## Built-in Variables

- `$0`: Entire line
- `$1, $2, ...`: Fields
- `NF`: Number of fields in current line
- `NR`: Current line number
- `FS`: Field separator (default: space)
- `OFS`: Output field separator
- `RS`: Record separator (default: newline)

## Flags

- `-F`: Set field separator
- `-v var=value`: Set variable
- `-f`: Read program from file

## Exit Codes

- `0`: Success
- `1`: Error in program
- `2`: Error in input

## Common Errors

- "syntax error" - Check quotes and braces
- Wrong column - Remember columns are 1-indexed
- Whitespace issues - Use -F to set explicit separator
