# grep

> Search for patterns in files. Essential for finding code, logs, and text.
> See also: rg, ag, ack, sed
> Keywords: search, find, pattern, regex, text, filter, match, lines

- Search for a string in a file:

`grep "pattern" file.txt`

- Search recursively in a directory:

`grep -r "pattern" /path/to/dir`

- Case-insensitive search:

`grep -i "pattern" file.txt`

- Show line numbers:

`grep -n "pattern" file.txt`

- Show only filenames with matches:

`grep -l "pattern" *.txt`

- Count matches per file:

`grep -c "pattern" *.txt`

- Invert match (lines NOT containing pattern):

`grep -v "pattern" file.txt`

- Search for whole words only:

`grep -w "word" file.txt`

- Show context lines around match:

`grep -C 3 "pattern" file.txt`

- Use extended regex (for +, ?, |, etc.):

`grep -E "pattern1|pattern2" file.txt`

- Search for fixed string (no regex):

`grep -F "literal.string" file.txt`

- Exclude directories from recursive search:

`grep -r --exclude-dir=node_modules "pattern" .`

- Search only specific file types:

`grep -r --include="*.py" "pattern" .`

## Flags

- `-r, --recursive`: Search recursively
- `-i, --ignore-case`: Case-insensitive
- `-n, --line-number`: Show line numbers
- `-l, --files-with-matches`: Only show filenames
- `-c, --count`: Count matches
- `-v, --invert-match`: Invert match
- `-w, --word-regexp`: Match whole words
- `-E, --extended-regexp`: Extended regex
- `-F, --fixed-strings`: Literal string match
- `-A NUM`: Show NUM lines after match
- `-B NUM`: Show NUM lines before match
- `-C NUM`: Show NUM lines of context
- `-o, --only-matching`: Print only matched parts
- `-q, --quiet`: Quiet mode (for scripting)

## Exit Codes

- `0`: Match found
- `1`: No match found
- `2`: Error occurred

## Common Errors

- "Binary file matches" - Use --text or -a to force text mode, or use grep -I to skip binary
- Regex not matching - Remember to escape special chars or use -F for literal strings
- Missing matches - Check if you need -r for recursion or -i for case
