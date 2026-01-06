# kill

> Send signals to processes. Terminate or control processes.
> See also: pkill, killall, ps
> Keywords: terminate, stop, signal, process, pid, end

- Terminate a process gracefully (SIGTERM):

`kill 1234`

- Force kill a process (SIGKILL):

`kill -9 1234`

- Send SIGTERM explicitly:

`kill -15 1234`

- Kill multiple processes:

`kill 1234 5678 9012`

- Send SIGHUP (reload config):

`kill -HUP 1234`

- Send SIGSTOP (pause process):

`kill -STOP 1234`

- Send SIGCONT (resume process):

`kill -CONT 1234`

- List all available signals:

`kill -l`

- Kill by job number (background job):

`kill %1`

- Kill all processes you own (DANGEROUS):

`kill -9 -1`

## Common Signals

- `SIGTERM (15)`: Graceful termination (default)
- `SIGKILL (9)`: Force kill (cannot be caught)
- `SIGHUP (1)`: Hangup, often used to reload config
- `SIGINT (2)`: Interrupt (like Ctrl+C)
- `SIGSTOP (19)`: Stop/pause process
- `SIGCONT (18)`: Continue stopped process
- `SIGUSR1 (10)`: User-defined signal 1
- `SIGUSR2 (12)`: User-defined signal 2

## Flags

- `-s signal`: Specify signal by name
- `-l`: List all signal names
- `-9`: Send SIGKILL (force)
- `-15`: Send SIGTERM (graceful)

## Exit Codes

- `0`: Success
- `1`: Error (process not found or permission denied)

## Common Errors

- "No such process" - Process already ended or wrong PID
- "Operation not permitted" - Need root or own the process
- Process won't die with SIGTERM - Use SIGKILL (-9)
- Zombie process - Can't kill, must kill parent

## Related Commands

- `pkill nginx`: Kill by name
- `killall nginx`: Kill all with name
- `pgrep nginx`: Find PID by name
