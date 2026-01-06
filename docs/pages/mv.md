# mv

> Move or rename files and directories.
> See also: cp, rm, rename
> Keywords: move, rename, relocate, files

- Rename a file:

`mv old_name.txt new_name.txt`

- Move a file to a directory:

`mv file.txt /path/to/directory/`

- Move multiple files to a directory:

`mv file1.txt file2.txt /path/to/directory/`

- Move a directory:

`mv source_dir/ /path/to/destination/`

- Move interactively (prompt before overwrite):

`mv -i source.txt destination.txt`

- Move without overwriting existing files:

`mv -n source.txt destination.txt`

- Move verbose (show what's being moved):

`mv -v source.txt destination.txt`

- Force move (overwrite without prompting):

`mv -f source.txt destination.txt`

- Move and create backup of destination:

`mv -b source.txt destination.txt`

## Flags

- `-i, --interactive`: Prompt before overwrite
- `-n, --no-clobber`: Don't overwrite existing files
- `-f, --force`: Don't prompt before overwriting
- `-v, --verbose`: Explain what is being done
- `-b`: Make backup of each existing destination file
- `-u, --update`: Move only when source is newer

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Common Errors

- "cannot move to itself" - Source and destination are the same
- "Directory not empty" - Can't overwrite non-empty directory
- "Permission denied" - No write permission on source or destination
- "No such file or directory" - Source doesn't exist
