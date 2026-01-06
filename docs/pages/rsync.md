# rsync

> Fast, versatile file synchronization. Better than scp for large transfers.
> See also: scp, cp, rclone
> Keywords: sync, copy, transfer, backup, remote, incremental, mirror

- Sync directory locally:

`rsync -av source/ destination/`

- Sync to remote host:

`rsync -av source/ user@host:/path/to/destination/`

- Sync from remote host:

`rsync -av user@host:/path/to/source/ destination/`

- Sync with delete (mirror exactly):

`rsync -av --delete source/ destination/`

- Dry run (show what would happen):

`rsync -av --dry-run source/ destination/`

- Show progress:

`rsync -av --progress source/ destination/`

- Use SSH on different port:

`rsync -av -e "ssh -p 2222" source/ user@host:/path/`

- Exclude files/patterns:

`rsync -av --exclude='*.log' source/ destination/`

- Exclude from file:

`rsync -av --exclude-from='exclude.txt' source/ destination/`

- Compress during transfer:

`rsync -avz source/ user@host:/path/`

- Limit bandwidth (KB/s):

`rsync -av --bwlimit=1000 source/ destination/`

- Resume interrupted transfer:

`rsync -av --partial source/ destination/`

- Preserve hard links:

`rsync -avH source/ destination/`

## IMPORTANT: Trailing Slash Behavior

- `rsync source destination/` → Creates `destination/source/`
- `rsync source/ destination/` → Copies contents into `destination/`

Always use trailing slash on source to copy contents!

## Flags

- `-a, --archive`: Archive mode (preserves everything)
- `-v, --verbose`: Verbose output
- `-z, --compress`: Compress during transfer
- `-P`: Same as --partial --progress
- `--progress`: Show progress
- `--partial`: Keep partial files
- `--delete`: Delete extraneous files from destination
- `--dry-run`: Show what would be done
- `-e`: Specify remote shell
- `--exclude`: Exclude pattern
- `--include`: Include pattern
- `--bwlimit`: Limit bandwidth
- `-H`: Preserve hard links
- `-n`: Dry run

## Exit Codes

- `0`: Success
- `1`: Syntax/usage error
- `2`: Protocol incompatibility
- `3`: File selection error
- `5`: Error starting client-server protocol
- `10`: Error in socket I/O
- `11`: Error in file I/O
- `12`: Error in rsync protocol data
- `23`: Partial transfer due to error
- `24`: Partial transfer due to vanished source files

## Common Errors

- "Permission denied" - Check file permissions or SSH access
- "No such file or directory" - Check source path
- Unexpected behavior - Check trailing slashes on paths
