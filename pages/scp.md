# scp

> Secure copy - transfer files over SSH.
> See also: rsync, sftp, ssh
> Keywords: copy, transfer, remote, secure, upload, download

- Copy file to remote host:

`scp file.txt user@host:/path/to/destination/`

- Copy file from remote host:

`scp user@host:/path/to/file.txt ./`

- Copy directory recursively:

`scp -r directory/ user@host:/path/to/destination/`

- Use specific SSH port:

`scp -P 2222 file.txt user@host:/path/`

- Use specific SSH key:

`scp -i ~/.ssh/mykey.pem file.txt user@host:/path/`

- Copy between two remote hosts:

`scp user1@host1:/file user2@host2:/path/`

- Preserve file attributes:

`scp -p file.txt user@host:/path/`

- Limit bandwidth (in Kbit/s):

`scp -l 1000 file.txt user@host:/path/`

- Verbose output:

`scp -v file.txt user@host:/path/`

- Compress during transfer:

`scp -C largefile.txt user@host:/path/`

- Copy multiple files:

`scp file1.txt file2.txt user@host:/path/`

- Use jump host:

`scp -o ProxyJump=jumphost file.txt user@destination:/path/`

## Flags

- `-r`: Recursive (for directories)
- `-P`: Port (note uppercase P, unlike ssh)
- `-i`: Identity file (key)
- `-p`: Preserve modification times and modes
- `-C`: Enable compression
- `-l`: Limit bandwidth (Kbit/s)
- `-v`: Verbose mode
- `-q`: Quiet mode
- `-o`: SSH option

## Exit Codes

- `0`: Success
- `1`: General error
- Other SSH exit codes apply

## Common Errors

- "No such file or directory" - Remote path doesn't exist
- "Permission denied" - Wrong permissions or auth failure
- "scp: not a regular file" - Use -r for directories
- Consider using rsync for large/incremental transfers

## Note

For new projects, consider using `rsync` or `sftp` instead - scp has some deprecated features and rsync is more efficient for large transfers.
