# rm

> Remove files and directories. DANGEROUS - no trash, files are gone forever.
> See also: rmdir, trash, shred
> Keywords: remove, delete, files, erase

- Remove a file:

`rm file.txt`

- Remove multiple files:

`rm file1.txt file2.txt file3.txt`

- Remove a directory and all its contents (DANGEROUS):

`rm -r directory/`

- Remove forcefully without prompting (VERY DANGEROUS):

`rm -rf directory/`

- Remove interactively (prompt for each file):

`rm -i file.txt`

- Remove verbose (show what's being deleted):

`rm -v file.txt`

- Remove files matching a pattern:

`rm *.tmp`

- Remove all files in directory but keep directory:

`rm -r directory/*`

- Dry run - show what would be deleted (using find):

`find . -name "*.tmp" -print`

## Flags

- `-r, -R, --recursive`: Remove directories and their contents
- `-f, --force`: Ignore nonexistent files, never prompt
- `-i`: Prompt before every removal
- `-I`: Prompt once before removing more than three files
- `-v, --verbose`: Explain what is being done
- `-d, --dir`: Remove empty directories

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Common Errors

- "Is a directory" - Use -r to remove directories
- "Permission denied" - No write permission on parent directory
- "Directory not empty" - Use -r for non-empty directories
- "No such file or directory" - File doesn't exist (use -f to ignore)

## DANGER ZONE

- `rm -rf /` - DO NOT RUN - destroys entire system
- `rm -rf *` - Deletes everything in current directory
- `rm -rf ~` - Deletes entire home directory
- Always double-check paths before running rm -rf
