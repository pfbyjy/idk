# free

> Display amount of free and used memory in the system.
> See also: top, vmstat, /proc/meminfo
> Keywords: memory, ram, usage, available, swap, system

- Show memory usage:

`free`

- Show in human-readable format:

`free -h`

- Show in megabytes:

`free -m`

- Show in gigabytes:

`free -g`

- Show total line:

`free -t`

- Continuous monitoring (every 2 seconds):

`free -s 2`

- Show with count limit:

`free -s 2 -c 5`

- Show wide output (buffers and cache separate):

`free -w`

## Output Columns

- `total`: Total installed memory
- `used`: Used memory
- `free`: Unused memory
- `shared`: Memory used by tmpfs
- `buff/cache`: Memory used for buffers and cache
- `available`: Memory available for starting new apps

## Understanding Memory

- `available` is what you should look at, not `free`
- Linux uses "free" memory for disk cache
- High `buff/cache` is normal and good
- Low `available` means you're running out of memory
- Swap usage indicates memory pressure

## Flags

- `-h, --human`: Human-readable output
- `-b, --bytes`: Show in bytes
- `-k, --kibi`: Show in kibibytes (default)
- `-m, --mebi`: Show in mebibytes
- `-g, --gibi`: Show in gibibytes
- `-t, --total`: Show total line
- `-s N, --seconds N`: Continuous display every N seconds
- `-c N, --count N`: Display N times, then exit
- `-w, --wide`: Wide output

## Exit Codes

- `0`: Success

## Common Errors

- "free" shows lots of used memory - Check "available", cache is reclaimable
- Swap being used - Not always bad, but check if available is low
