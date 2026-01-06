# tr

> Translate or delete characters. Works on stdin only.
> See also: sed, awk
> Keywords: translate, replace, characters, delete, squeeze, case

- Convert lowercase to uppercase:

`echo "hello" | tr 'a-z' 'A-Z'`

- Convert uppercase to lowercase:

`echo "HELLO" | tr 'A-Z' 'a-z'`

- Replace spaces with newlines:

`echo "a b c" | tr ' ' '\n'`

- Delete specific characters:

`echo "hello123" | tr -d '0-9'`

- Delete all digits:

`tr -d '[:digit:]' < file.txt`

- Squeeze repeated characters:

`echo "hellooooo" | tr -s 'o'`

- Replace non-alphanumeric with newlines:

`tr -cs '[:alnum:]' '\n' < file.txt`

- Remove all whitespace:

`tr -d '[:space:]' < file.txt`

- Convert Windows line endings to Unix:

`tr -d '\r' < windows.txt > unix.txt`

- ROT13 encoding:

`echo "hello" | tr 'A-Za-z' 'N-ZA-Mn-za-m'`

- Replace multiple characters:

`echo "abc" | tr 'abc' 'xyz'`

- Complement character set (delete all except):

`tr -cd '[:print:]' < file.txt`

## Character Classes

- `[:alnum:]`: Alphanumeric
- `[:alpha:]`: Letters
- `[:digit:]`: Digits
- `[:space:]`: Whitespace
- `[:lower:]`: Lowercase
- `[:upper:]`: Uppercase
- `[:punct:]`: Punctuation
- `[:print:]`: Printable characters

## Flags

- `-d, --delete`: Delete characters in SET1
- `-s, --squeeze-repeats`: Squeeze repeated characters to single
- `-c, -C, --complement`: Use complement of SET1
- `-t, --truncate-set1`: Truncate SET1 to length of SET2

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- "extra operand" - Too many arguments, check quoting
- Only works on stdin - Can't give filename directly, use < redirect
- Character not translated - Check if using right case in set
