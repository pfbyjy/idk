# find

> Search for files in a directory hierarchy. Extremely powerful.
> See also: locate, fd, grep
> Keywords: search, find, files, locate, pattern, recursive

- Find files by name:

`find /path -name "filename.txt"`

- Find files by name (case-insensitive):

`find /path -iname "filename.txt"`

- Find files by extension:

`find . -name "*.py"`

- Find directories only:

`find . -type d -name "src"`

- Find files only:

`find . -type f -name "*.js"`

- Find files modified in last 24 hours:

`find . -mtime -1`

- Find files modified more than 7 days ago:

`find . -mtime +7`

- Find files larger than 100MB:

`find . -size +100M`

- Find empty files:

`find . -empty`

- Find and delete files:

`find . -name "*.tmp" -delete`

- Find and execute command on each file:

`find . -name "*.txt" -exec cat {} \;`

- Find and execute command with all files at once:

`find . -name "*.txt" -exec cat {} +`

- Find excluding a directory:

`find . -path ./node_modules -prune -o -name "*.js" -print`

- Find files with specific permissions:

`find . -perm 644`

- Find files owned by user:

`find . -user username`

- Find and print with null separator (for xargs):

`find . -name "*.txt" -print0 | xargs -0 rm`

## Flags

- `-name`: Match filename pattern
- `-iname`: Case-insensitive name match
- `-type`: File type (f=file, d=directory, l=symlink)
- `-mtime`: Modified time in days (+n=more than, -n=less than)
- `-mmin`: Modified time in minutes
- `-size`: File size (+100M=larger, -100M=smaller)
- `-empty`: Empty files or directories
- `-delete`: Delete matching files
- `-exec`: Execute command on each match
- `-print0`: Print with null separator
- `-maxdepth`: Limit search depth
- `-path`: Match full path
- `-prune`: Don't descend into directory

## Exit Codes

- `0`: Success (even if no files found)
- `1`: Error occurred

## Common Errors

- "Permission denied" on some dirs - Normal for system dirs, ignore with 2>/dev/null
- "missing argument to -exec" - Need {} \; or {} + at end
- Finding too many files - Use -maxdepth to limit
