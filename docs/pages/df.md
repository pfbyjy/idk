# df

> Report file system disk space usage.
> See also: du, lsblk, fdisk
> Keywords: disk, space, usage, free, filesystem, storage, mount

- Show disk usage for all filesystems:

`df`

- Show in human-readable format:

`df -h`

- Show specific filesystem:

`df -h /dev/sda1`

- Show usage for a path:

`df -h /home`

- Show filesystem type:

`df -T`

- Show only local filesystems:

`df -l`

- Show inodes instead of blocks:

`df -i`

- Show in 1K blocks:

`df -k`

- Show in MB:

`df -m`

- Exclude specific filesystem types:

`df -x tmpfs -x devtmpfs`

- Show total:

`df -h --total`

## Output Columns

- `Filesystem`: Device or mount point
- `Size`: Total size
- `Used`: Space used
- `Avail`: Space available
- `Use%`: Percentage used
- `Mounted on`: Mount point

## Flags

- `-h, --human-readable`: Human-readable sizes (K, M, G)
- `-H`: Human-readable with powers of 1000
- `-T, --print-type`: Show filesystem type
- `-i, --inodes`: Show inode information
- `-l, --local`: Only local filesystems
- `-t, --type`: Only show specific filesystem type
- `-x, --exclude-type`: Exclude filesystem type
- `--total`: Add total row

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- "No such file or directory" - Path doesn't exist
- Shows 100% but du shows less - Reserved space for root (usually 5%)
- Filesystem not showing - May be a pseudo-filesystem
