# mkdir

> Create directories.
> See also: rmdir, touch, install
> Keywords: make, create, directory, folder

- Create a directory:

`mkdir new_directory`

- Create multiple directories:

`mkdir dir1 dir2 dir3`

- Create parent directories as needed:

`mkdir -p path/to/nested/directory`

- Create with specific permissions:

`mkdir -m 755 new_directory`

- Create verbose (show what's created):

`mkdir -v new_directory`

- Create directory structure for a project:

`mkdir -p project/{src,tests,docs,bin}`

- Create dated directory:

`mkdir "$(date +%Y-%m-%d)"`

## Flags

- `-p, --parents`: Create parent directories as needed, no error if exists
- `-m, --mode`: Set file mode (permissions)
- `-v, --verbose`: Print a message for each created directory

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Common Errors

- "File exists" - Directory already exists (use -p to ignore)
- "No such file or directory" - Parent doesn't exist (use -p)
- "Permission denied" - Can't write to parent directory
