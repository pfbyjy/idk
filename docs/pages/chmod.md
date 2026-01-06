# chmod

> Change file mode (permissions).
> See also: chown, chgrp, stat, umask
> Keywords: permissions, mode, access, read, write, execute, security

- Make file executable:

`chmod +x script.sh`

- Remove execute permission:

`chmod -x script.sh`

- Set exact permissions (owner rwx, group rx, others rx):

`chmod 755 file`

- Set exact permissions (owner rw, group r, others r):

`chmod 644 file`

- Owner can read/write, no one else can access:

`chmod 600 file`

- Make directory accessible:

`chmod 755 directory/`

- Recursive change:

`chmod -R 755 directory/`

- Add write permission for group:

`chmod g+w file`

- Remove all permissions for others:

`chmod o-rwx file`

- Copy permissions from another file:

`chmod --reference=source_file target_file`

- Set permissions using symbolic notation:

`chmod u=rwx,g=rx,o=r file`

- Make executable by owner only:

`chmod u+x,go-x script.sh`

## Permission Numbers

- `7`: rwx (read + write + execute = 4+2+1)
- `6`: rw- (read + write = 4+2)
- `5`: r-x (read + execute = 4+1)
- `4`: r-- (read only)
- `3`: -wx (write + execute)
- `2`: -w- (write only)
- `1`: --x (execute only)
- `0`: --- (no permissions)

## Common Permission Patterns

- `755`: Executable scripts, directories
- `644`: Regular files
- `600`: Private files (SSH keys)
- `700`: Private directories/scripts
- `777`: Everyone can do everything (AVOID)

## Symbolic Mode

- `u`: User (owner)
- `g`: Group
- `o`: Others
- `a`: All (u+g+o)
- `+`: Add permission
- `-`: Remove permission
- `=`: Set exact permission

## Flags

- `-R, --recursive`: Change recursively
- `-v, --verbose`: Output for every file processed
- `--reference`: Use another file's permissions

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- "Operation not permitted" - Need to own file or be root
- Script won't execute - Needs +x permission
- Web files not loading - Check directory has +x for traversal
