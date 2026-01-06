# tail

> Output the last part of files. Essential for log monitoring.
> See also: head, less, watch
> Keywords: last, lines, end, bottom, logs, follow, stream

- Show last 10 lines of a file (default):

`tail file.txt`

- Show last N lines:

`tail -n 20 file.txt`

- Show last N bytes:

`tail -c 100 file.txt`

- Follow a file (watch for new content) - Ctrl+C to stop:

`tail -f logfile.log`

- Follow multiple files:

`tail -f log1.log log2.log`

- Follow and retry if file is recreated:

`tail -F logfile.log`

- Show lines starting from line N:

`tail -n +100 file.txt`

- Follow and show last 50 lines:

`tail -n 50 -f logfile.log`

- Follow with process termination when process dies:

`tail -f --pid=12345 logfile.log`

## Flags

- `-n, --lines`: Output last N lines (or +N for starting from line N)
- `-c, --bytes`: Output last N bytes
- `-f, --follow`: Output appended data as file grows
- `-F`: Same as -f but retry if file is recreated
- `--pid`: With -f, terminate after process ID dies
- `-q, --quiet`: Never print headers
- `-v, --verbose`: Always print headers

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Common Errors

- "cannot open for reading" - File doesn't exist or no permission
- Follow not working - File might be buffered, try -F
