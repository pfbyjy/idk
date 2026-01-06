# ssh

> Secure Shell - connect to remote machines securely.
> See also: scp, rsync, sftp, ssh-keygen
> Keywords: remote, connect, shell, login, secure, tunnel, server

- Connect to remote host:

`ssh user@hostname`

- Connect on different port:

`ssh -p 2222 user@hostname`

- Connect with specific key:

`ssh -i ~/.ssh/mykey.pem user@hostname`

- Run command on remote host:

`ssh user@hostname "ls -la /var/log"`

- Enable X11 forwarding:

`ssh -X user@hostname`

- Local port forward (access remote service locally):

`ssh -L 8080:localhost:80 user@hostname`

- Remote port forward (expose local service to remote):

`ssh -R 8080:localhost:80 user@hostname`

- Dynamic port forward (SOCKS proxy):

`ssh -D 1080 user@hostname`

- SSH tunnel in background:

`ssh -fN -L 8080:localhost:80 user@hostname`

- Keep connection alive:

`ssh -o ServerAliveInterval=60 user@hostname`

- Use jump host (bastion):

`ssh -J jumphost user@destination`

- Verbose output for debugging:

`ssh -v user@hostname`

- Copy SSH key to remote (enable passwordless):

`ssh-copy-id user@hostname`

- Run with pseudo-terminal:

`ssh -t user@hostname "sudo command"`

## Flags

- `-p`: Port number
- `-i`: Identity file (private key)
- `-L`: Local port forward
- `-R`: Remote port forward
- `-D`: Dynamic port forward (SOCKS)
- `-N`: No remote command (for tunnels)
- `-f`: Background after authentication
- `-t`: Force pseudo-terminal
- `-v`: Verbose mode (-vv, -vvv for more)
- `-X`: X11 forwarding
- `-J`: Jump host
- `-o`: Set option

## Config File (~/.ssh/config)

```
Host myserver
    HostName server.example.com
    User myuser
    Port 22
    IdentityFile ~/.ssh/mykey
```

Then just: `ssh myserver`

## Exit Codes

- `0`: Success
- `1`: Generic error
- `255`: SSH error

## Common Errors

- "Permission denied (publickey)" - Wrong key, or need password auth
- "Connection refused" - SSH not running or firewall blocking
- "Host key verification failed" - Remove old key from known_hosts
- "Connection timed out" - Network issue or wrong host
