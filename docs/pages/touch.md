# touch

> Create empty files or update file timestamps.
> See also: mkdir, stat, date
> Keywords: create, file, empty, timestamp, modify

- Create an empty file:

`touch newfile.txt`

- Create multiple files:

`touch file1.txt file2.txt file3.txt`

- Update modification time to now:

`touch existing_file.txt`

- Set specific modification time:

`touch -t 202401151200 file.txt`

- Set time to same as another file:

`touch -r reference_file.txt target_file.txt`

- Create file only if it doesn't exist:

`touch -c nonexistent.txt`

- Update only access time:

`touch -a file.txt`

- Update only modification time:

`touch -m file.txt`

## Flags

- `-a`: Change only access time
- `-m`: Change only modification time
- `-c, --no-create`: Don't create file if it doesn't exist
- `-t`: Use specified time [[CC]YY]MMDDhhmm[.ss]
- `-r, --reference`: Use time from another file
- `-d, --date`: Parse string and use as time

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Common Errors

- "Permission denied" - Can't write to directory
- "No such file or directory" - Parent directory doesn't exist
