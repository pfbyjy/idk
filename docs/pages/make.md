# make

> Build automation tool. Execute tasks defined in Makefile.
> See also: cmake, ninja, just
> Keywords: build, compile, makefile, automation, target, dependency

- Run default target:

`make`

- Run specific target:

`make build`

- Run with parallel jobs:

`make -j4`

- Run with maximum parallelism:

`make -j$(nproc)`

- Dry run (show commands without executing):

`make -n`

- Use different Makefile:

`make -f custom.mk`

- Pass variable:

`make CFLAGS="-O2"`

- Keep going on errors:

`make -k`

- Print database (all rules):

`make -p`

- Show why target is being made:

`make --debug`

- Change directory before reading Makefile:

`make -C /path/to/project`

- Remake all targets unconditionally:

`make -B`

- Silent mode (no command echo):

`make -s`

## Flags

- `-j N`: Run N jobs in parallel
- `-n, --dry-run`: Don't run commands, just print
- `-f FILE`: Use FILE as Makefile
- `-C DIR`: Change to directory before running
- `-k, --keep-going`: Continue on errors
- `-B, --always-make`: Remake all targets
- `-s, --silent`: Don't echo commands
- `-p, --print-data-base`: Print all rules and variables
- `--debug`: Print debugging info

## Common Targets

- `make`: Default target (usually `all`)
- `make all`: Build everything
- `make clean`: Remove built files
- `make install`: Install to system
- `make test`: Run tests
- `make help`: Show available targets

## Makefile Basics

```makefile
target: dependencies
	command  # MUST be tab, not spaces

.PHONY: clean  # Not a file
clean:
	rm -f *.o
```

## Exit Codes

- `0`: Success
- `1`: Error from make itself
- `2`: Error from executed command

## Common Errors

- "No targets" - No Makefile found or no default target
- "missing separator" - Use tabs, not spaces before commands
- "No rule to make target" - Missing dependency or file
- "Nothing to be done" - Target already up to date
