# tar

> Archive files. Create, extract, and manipulate tar archives.
> See also: gzip, zip, 7z
> Keywords: archive, compress, extract, backup, tarball, bundle

- Create archive:

`tar -cvf archive.tar directory/`

- Create gzip compressed archive:

`tar -czvf archive.tar.gz directory/`

- Create bzip2 compressed archive:

`tar -cjvf archive.tar.bz2 directory/`

- Create xz compressed archive:

`tar -cJvf archive.tar.xz directory/`

- Extract archive:

`tar -xvf archive.tar`

- Extract gzip archive:

`tar -xzvf archive.tar.gz`

- Extract to specific directory:

`tar -xvf archive.tar -C /destination/`

- List contents without extracting:

`tar -tvf archive.tar`

- Extract specific file:

`tar -xvf archive.tar path/to/file`

- Add files to existing archive:

`tar -rvf archive.tar newfile.txt`

- Exclude files:

`tar -cvf archive.tar --exclude='*.log' directory/`

- Exclude directory:

`tar -cvf archive.tar --exclude='node_modules' directory/`

- Create from file list:

`tar -cvf archive.tar -T filelist.txt`

- Extract with strip (remove leading path components):

`tar -xvf archive.tar --strip-components=1`

- Preserve permissions:

`tar -cvpf archive.tar directory/`

## Mnemonic: tar flags

- **c** - Create
- **x** - eXtract
- **t** - lisT
- **v** - Verbose
- **f** - File (archive name comes next)
- **z** - gZip
- **j** - bzip2 (j for bz2)
- **J** - xz

## Common Patterns

- `.tar` - Uncompressed archive
- `.tar.gz` or `.tgz` - Gzip compressed
- `.tar.bz2` - Bzip2 compressed
- `.tar.xz` - XZ compressed (best compression)

## Flags

- `-c, --create`: Create archive
- `-x, --extract`: Extract archive
- `-t, --list`: List contents
- `-v, --verbose`: Verbose output
- `-f, --file`: Archive file name
- `-z, --gzip`: Use gzip compression
- `-j, --bzip2`: Use bzip2 compression
- `-J, --xz`: Use xz compression
- `-C, --directory`: Change to directory
- `-p, --preserve-permissions`: Preserve permissions
- `--exclude`: Exclude pattern
- `--strip-components`: Strip path components

## Exit Codes

- `0`: Success
- `1`: Some files differ
- `2`: Fatal error

## Common Errors

- "Cannot open: No such file" - Check archive path
- "Not in gzip format" - Use correct flag (-z, -j, or none)
- "Permission denied" - Need write access or run as root
- Order matters: `tar -cvf` not `tar -fvc`
