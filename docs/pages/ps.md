# ps

> Report process status. Show running processes.
> See also: top, htop, pgrep, kill
> Keywords: process, running, list, pid, status, memory, cpu

- Show processes for current user:

`ps`

- Show all processes (full format):

`ps aux`

- Show all processes (standard format):

`ps -ef`

- Show process tree:

`ps auxf`

- Show specific process by PID:

`ps -p 1234`

- Show processes by name:

`ps aux | grep nginx`

- Show processes for a user:

`ps -u username`

- Show processes with custom columns:

`ps -eo pid,ppid,cmd,%mem,%cpu`

- Show top memory consuming processes:

`ps aux --sort=-%mem | head`

- Show top CPU consuming processes:

`ps aux --sort=-%cpu | head`

- Show process tree for specific process:

`ps -ef --forest | grep -A5 nginx`

- Show threads:

`ps -eLf`

- Show only process IDs:

`ps -eo pid`

## Common Column Meanings

- `PID`: Process ID
- `PPID`: Parent process ID
- `USER`: Owner of process
- `%CPU`: CPU usage percentage
- `%MEM`: Memory usage percentage
- `VSZ`: Virtual memory size (KB)
- `RSS`: Resident set size (physical memory KB)
- `TTY`: Controlling terminal
- `STAT`: Process state (R=running, S=sleeping, Z=zombie)
- `START`: Start time
- `TIME`: CPU time used
- `COMMAND`: Command line

## Flags

- `a`: Show processes for all users
- `u`: Display user-oriented format
- `x`: Show processes without controlling terminal
- `-e`: Select all processes
- `-f`: Full format listing
- `-p`: Select by PID
- `-u`: Select by user
- `-o`: Custom output format
- `--sort`: Sort by column

## Exit Codes

- `0`: Success
- `1`: Error

## Common Errors

- "process not found" - Process may have ended
- Too many processes - Use grep or --sort to filter
