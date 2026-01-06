# chown

> Change file owner and group.
> See also: chmod, chgrp, stat
> Keywords: owner, permissions, group, ownership, user

- Change owner of a file:

`chown user file.txt`

- Change owner and group:

`chown user:group file.txt`

- Change only group:

`chown :group file.txt`

- Change owner recursively:

`chown -R user directory/`

- Change owner and group recursively:

`chown -R user:group directory/`

- Copy ownership from another file:

`chown --reference=source_file target_file`

- Change owner with verbose output:

`chown -v user file.txt`

- Change owner without following symlinks:

`chown -h user symlink`

- Change ownership from specific user only:

`chown --from=olduser newuser file.txt`

## Flags

- `-R, --recursive`: Operate recursively
- `-v, --verbose`: Output for every file processed
- `-c, --changes`: Like verbose but only report changes
- `-h, --no-dereference`: Affect symbolic links instead of referenced file
- `--reference`: Use another file's ownership
- `--from`: Only change if current owner matches

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- "Operation not permitted" - Only root can change ownership
- "invalid user" - User doesn't exist
- "invalid group" - Group doesn't exist
- Use numeric IDs if user/group name doesn't exist

## Common Patterns

- `chown www-data:www-data /var/www/` - Web server files
- `chown -R $USER:$USER ~/` - Reclaim home directory
- `chown root:root /etc/important` - System files
