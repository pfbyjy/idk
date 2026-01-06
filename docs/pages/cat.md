# cat

> Concatenate and print files. Display file contents to stdout.
> See also: less, head, tail, bat
> Keywords: print, display, file, contents, concatenate, read, show

- Display contents of a file:

`cat file.txt`

- Display contents of multiple files:

`cat file1.txt file2.txt`

- Display with line numbers:

`cat -n file.txt`

- Display with line numbers (non-empty lines only):

`cat -b file.txt`

- Display showing non-printing characters:

`cat -v file.txt`

- Display showing tabs as ^I:

`cat -T file.txt`

- Display showing line endings as $:

`cat -E file.txt`

- Show all special characters (-vET combined):

`cat -A file.txt`

- Concatenate files into one:

`cat file1.txt file2.txt > combined.txt`

- Append file to another:

`cat file2.txt >> file1.txt`

- Create a file with content (heredoc):

`cat << 'EOF' > file.txt`

- Number only non-blank lines and squeeze blank lines:

`cat -bs file.txt`

## Flags

- `-n, --number`: Number all output lines
- `-b, --number-nonblank`: Number non-empty output lines
- `-s, --squeeze-blank`: Squeeze multiple blank lines into one
- `-v, --show-nonprinting`: Show non-printing characters
- `-E, --show-ends`: Display $ at end of each line
- `-T, --show-tabs`: Display TAB as ^I
- `-A, --show-all`: Equivalent to -vET

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Common Errors

- "No such file or directory" - File doesn't exist
- "Is a directory" - Can't cat a directory
- "Permission denied" - No read permission
