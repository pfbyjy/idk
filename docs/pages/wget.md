# wget

> Download files from the web. Non-interactive network downloader.
> See also: curl, aria2c
> Keywords: download, http, https, ftp, web, file, url

- Download a file:

`wget https://example.com/file.zip`

- Download and save with different name:

`wget -O output.zip https://example.com/file.zip`

- Download to specific directory:

`wget -P /path/to/dir https://example.com/file.zip`

- Resume interrupted download:

`wget -c https://example.com/largefile.zip`

- Download in background:

`wget -b https://example.com/file.zip`

- Download quietly (no output):

`wget -q https://example.com/file.zip`

- Download multiple files:

`wget -i urls.txt`

- Download entire website:

`wget -r -l 2 https://example.com/`

- Mirror a website:

`wget --mirror -p --convert-links https://example.com/`

- Limit download speed:

`wget --limit-rate=200k https://example.com/file.zip`

- Download with authentication:

`wget --user=username --password=password https://example.com/file`

- Ignore SSL certificate errors:

`wget --no-check-certificate https://example.com/file`

- Set custom User-Agent:

`wget --user-agent="Mozilla/5.0" https://example.com/`

- Download only specific file types:

`wget -r -A "*.pdf" https://example.com/`

## Flags

- `-O`: Output to specific file
- `-P`: Save to directory
- `-c, --continue`: Resume download
- `-b, --background`: Run in background
- `-q, --quiet`: Quiet mode
- `-i`: Read URLs from file
- `-r, --recursive`: Recursive download
- `-l, --level`: Max recursion depth
- `--mirror`: Mirror website
- `--limit-rate`: Limit download speed
- `--user, --password`: Authentication
- `--no-check-certificate`: Skip SSL verification
- `-A, --accept`: Accept file patterns
- `-R, --reject`: Reject file patterns

## Exit Codes

- `0`: Success
- `1`: Generic error
- `2`: Parse error
- `3`: File I/O error
- `4`: Network failure
- `5`: SSL verification failure
- `6`: Authentication failure
- `7`: Protocol error
- `8`: Server error

## Common Errors

- "Unable to resolve host" - DNS issue or typo in URL
- "Connection refused" - Server down or blocked
- "404 Not Found" - URL doesn't exist
- "SSL certificate problem" - Use --no-check-certificate (carefully)
