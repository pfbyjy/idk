# sudo

> Execute a command as another user (usually root).
> See also: su, doas, visudo
> Keywords: root, admin, superuser, elevate, privilege, permission

- Run command as root:

`sudo command`

- Run command as specific user:

`sudo -u username command`

- Start root shell:

`sudo -i`

- Start shell preserving environment:

`sudo -s`

- Edit file with root privileges:

`sudo nano /etc/hosts`

- Run last command as root:

`sudo !!`

- List allowed commands for current user:

`sudo -l`

- Extend sudo timeout:

`sudo -v`

- Invalidate sudo credentials (require password again):

`sudo -k`

- Run command preserving environment:

`sudo -E command`

- Set HOME to target user's home:

`sudo -H command`

- Run command in background:

`sudo -b command`

- Run as group:

`sudo -g groupname command`

## Flags

- `-u user`: Run as specified user
- `-i`: Login shell as root
- `-s`: Run shell (not login shell)
- `-l`: List allowed commands
- `-v`: Update timestamp (extend timeout)
- `-k`: Invalidate timestamp
- `-E`: Preserve environment
- `-H`: Set HOME to target's home
- `-b`: Run in background
- `-n`: Non-interactive (fail if password needed)
- `-A`: Use askpass helper

## Exit Codes

- Command's exit code on success
- `1`: Generic error
- `127`: Command not found

## Configuration

- Edit with `sudo visudo` (never edit /etc/sudoers directly)
- Per-user rules in `/etc/sudoers.d/`
- Syntax: `user host = (runas) commands`

## Common Errors

- "user is not in the sudoers file" - Add user to sudo group
- "sorry, try again" - Wrong password
- "command not found" - Command not in secure_path
- "unable to resolve host" - Hostname mismatch in /etc/hosts

## Security Tips

- Use `sudo` over `su` for audit trail
- Don't use `sudo -i` for single commands
- Check what you're running: `sudo -l`
