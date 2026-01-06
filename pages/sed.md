# sed

> Stream editor for filtering and transforming text.
> See also: awk, perl, tr
> Keywords: replace, substitute, text, transform, edit, regex, stream

- Replace first occurrence per line:

`sed 's/old/new/' file.txt`

- Replace all occurrences (global):

`sed 's/old/new/g' file.txt`

- Replace case-insensitive:

`sed 's/old/new/gi' file.txt`

- Edit file in place:

`sed -i 's/old/new/g' file.txt`

- Edit in place with backup:

`sed -i.bak 's/old/new/g' file.txt`

- Delete lines matching pattern:

`sed '/pattern/d' file.txt`

- Delete empty lines:

`sed '/^$/d' file.txt`

- Print only lines matching pattern:

`sed -n '/pattern/p' file.txt`

- Replace only on specific line:

`sed '5s/old/new/' file.txt`

- Replace on line range:

`sed '1,10s/old/new/g' file.txt`

- Insert line before match:

`sed '/pattern/i\new line' file.txt`

- Append line after match:

`sed '/pattern/a\new line' file.txt`

- Multiple operations:

`sed -e 's/foo/bar/g' -e 's/baz/qux/g' file.txt`

- Use different delimiter (useful for paths):

`sed 's|/old/path|/new/path|g' file.txt`

- Remove leading whitespace:

`sed 's/^[ \t]*//' file.txt`

- Remove trailing whitespace:

`sed 's/[ \t]*$//' file.txt`

## Flags

- `-i`: Edit files in place
- `-i.bak`: Edit in place and create backup with .bak extension
- `-n`: Suppress automatic printing
- `-e`: Add script/expression
- `-r` or `-E`: Use extended regex
- `-f`: Read script from file

## Regex Patterns

- `^`: Start of line
- `$`: End of line
- `.`: Any character
- `*`: Zero or more
- `\+`: One or more (need -E or escape)
- `[abc]`: Character class
- `\(...\)`: Capture group (use \1 to reference)

## Exit Codes

- `0`: Success
- `1`: Invalid command
- `2`: Input file error

## Common Errors

- "unterminated s command" - Missing closing delimiter
- Nothing replaced - Pattern didn't match, check regex
- Special chars not escaped - Escape . * [ ] etc.
