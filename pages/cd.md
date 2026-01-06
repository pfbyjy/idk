# cd

> Change the current working directory. Shell builtin, not a binary.
> See also: pushd, popd, pwd
> Keywords: change, directory, navigate, folder, path

- Go to a directory:

`cd /path/to/directory`

- Go to home directory:

`cd`

- Go to home directory (explicit):

`cd ~`

- Go to previous directory:

`cd -`

- Go up one directory:

`cd ..`

- Go up multiple directories:

`cd ../../..`

- Go to root directory:

`cd /`

- Go to directory with spaces in name:

`cd "path/with spaces/dir"`

- Go to another user's home directory:

`cd ~username`

## Exit Codes

- `0`: Success
- `1`: Directory doesn't exist or no permission

## Common Errors

- "No such file or directory" - Path doesn't exist
- "Not a directory" - Path is a file, not a directory
- "Permission denied" - No execute permission on directory
- No output on success - cd is silent when it works
