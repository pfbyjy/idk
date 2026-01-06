# less

> View file contents with pagination. Better than more.
> See also: more, cat, vim
> Keywords: view, read, paginate, scroll, browse, pager

- View a file:

`less file.txt`

- View with line numbers:

`less -N file.txt`

- View ignoring case in searches:

`less -i file.txt`

- View multiple files (use :n and :p to navigate):

`less file1.txt file2.txt`

- View output from a command:

`command | less`

- Start at a specific line:

`less +100 file.txt`

- Start at first match of pattern:

`less +/pattern file.txt`

- View with syntax highlighting (if source-highlight installed):

`less -R file.txt`

## Navigation (inside less)

- `Space` or `f`: Page down
- `b`: Page up
- `g`: Go to beginning
- `G`: Go to end
- `/pattern`: Search forward
- `?pattern`: Search backward
- `n`: Next search match
- `N`: Previous search match
- `q`: Quit
- `h`: Help
- `:n`: Next file
- `:p`: Previous file
- `F`: Follow mode (like tail -f)

## Flags

- `-N, --LINE-NUMBERS`: Show line numbers
- `-i, --ignore-case`: Case-insensitive search
- `-S, --chop-long-lines`: Don't wrap long lines
- `-R, --RAW-CONTROL-CHARS`: Show ANSI colors
- `-F, --quit-if-one-screen`: Exit if content fits one screen
- `-X, --no-init`: Don't clear screen on exit
- `+cmd`: Run command on startup

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- "file is a directory" - Use ls instead
- Search not finding - Check case, use -i for case-insensitive
