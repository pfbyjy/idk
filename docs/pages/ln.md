# ln

> Create links between files. Hard links or symbolic (soft) links.
> See also: readlink, cp, mv
> Keywords: link, symlink, symbolic, hard, alias

- Create a symbolic link (most common):

`ln -s /path/to/target link_name`

- Create a hard link:

`ln /path/to/target link_name`

- Create symbolic link with relative path:

`ln -sr target link_name`

- Create symbolic link, overwriting existing:

`ln -sf /path/to/target link_name`

- Create symbolic link to directory:

`ln -s /path/to/directory link_name`

- Create multiple links to files in a directory:

`ln -s /path/to/files/* /path/to/links/`

- Create hard link (backup without copying):

`ln original.txt backup.txt`

## Flags

- `-s, --symbolic`: Create symbolic link instead of hard link
- `-f, --force`: Remove existing destination files
- `-r, --relative`: Create relative symbolic links
- `-v, --verbose`: Print name of each linked file
- `-n, --no-dereference`: Treat link destination as file

## Hard Links vs Symbolic Links

Hard links:
- Point to same inode (same data on disk)
- Can't cross filesystems
- Can't link to directories
- Original can be deleted, link still works

Symbolic links:
- Point to path (like a shortcut)
- Can cross filesystems
- Can link to directories
- Break if original is deleted/moved

## Exit Codes

- `0`: Success
- `1`: Error occurred

## Common Errors

- "File exists" - Link already exists (use -f to overwrite)
- "hard link not allowed for directory" - Use -s for directories
- "Invalid cross-device link" - Hard links can't cross filesystems
