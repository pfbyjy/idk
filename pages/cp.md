# cp

> Copy files and directories.
> See also: mv, rsync, scp
> Keywords: copy, duplicate, files, backup

- Copy a file to another location:

`cp source.txt destination.txt`

- Copy a file to a directory:

`cp file.txt /path/to/directory/`

- Copy multiple files to a directory:

`cp file1.txt file2.txt /path/to/directory/`

- Copy a directory recursively:

`cp -r source_dir/ destination_dir/`

- Copy and preserve attributes (mode, ownership, timestamps):

`cp -p source.txt destination.txt`

- Copy interactively (prompt before overwrite):

`cp -i source.txt destination.txt`

- Copy without overwriting existing files:

`cp -n source.txt destination.txt`

- Copy verbose (show what's being copied):

`cp -v source.txt destination.txt`

- Copy and create backup of destination if it exists:

`cp -b source.txt destination.txt`

- Copy following symbolic links:

`cp -L source_link destination`

## Flags

- `-r, -R, --recursive`: Copy directories recursively
- `-i, --interactive`: Prompt before overwrite
- `-n, --no-clobber`: Don't overwrite existing files
- `-v, --verbose`: Explain what is being done
- `-p`: Preserve mode, ownership, timestamps
- `-a, --archive`: Same as -dR --preserve=all (for backups)
- `-u, --update`: Copy only when source is newer
- `-f, --force`: Remove destination file if needed
- `-L, --dereference`: Follow symbolic links in source

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Common Errors

- "omitting directory" - Use -r to copy directories
- "cannot overwrite" - Destination exists and is protected
- "No space left on device" - Disk full
- "Permission denied" - Can't read source or write to destination
