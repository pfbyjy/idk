# top

> Display real-time system processes. Interactive process viewer.
> See also: htop, ps, vmstat
> Keywords: process, monitor, cpu, memory, real-time, system

- Start top:

`top`

- Sort by memory usage:

`top -o %MEM`

- Sort by CPU usage:

`top -o %CPU`

- Show specific user's processes:

`top -u username`

- Batch mode (non-interactive, good for scripts):

`top -b -n 1`

- Show specific number of processes:

`top -n 1 -b | head -20`

- Update every 5 seconds:

`top -d 5`

- Show specific process:

`top -p 1234`

- Show multiple PIDs:

`top -p 1234,5678,9012`

## Interactive Commands (while running)

- `q`: Quit
- `h`: Help
- `k`: Kill a process (enter PID)
- `r`: Renice (change priority)
- `M`: Sort by memory
- `P`: Sort by CPU
- `T`: Sort by time
- `u`: Filter by user
- `c`: Toggle full command path
- `1`: Toggle individual CPU cores
- `m`: Toggle memory display mode
- `t`: Toggle task/cpu display
- `Space`: Refresh immediately
- `W`: Write config to ~/.toprc

## Display Fields

- `PID`: Process ID
- `USER`: Process owner
- `PR`: Priority
- `NI`: Nice value (-20 to 19)
- `VIRT`: Virtual memory
- `RES`: Resident memory (physical)
- `SHR`: Shared memory
- `S`: Process state
- `%CPU`: CPU percentage
- `%MEM`: Memory percentage
- `TIME+`: CPU time

## Flags

- `-d`: Delay between updates (seconds)
- `-n`: Number of iterations
- `-b`: Batch mode
- `-u`: Show only user's processes
- `-p`: Monitor specific PIDs
- `-o`: Sort by field

## Exit Codes

- `0`: Success

## Common Errors

- Can't see all processes - Run as root for full visibility
- Display messed up - Press 'W' to save config, or delete ~/.toprc
