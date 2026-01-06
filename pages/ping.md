# ping

> Send ICMP ECHO_REQUEST to network hosts. Test network connectivity.
> See also: traceroute, mtr, nc
> Keywords: network, test, connectivity, latency, host, icmp

- Ping a host:

`ping example.com`

- Ping with count limit:

`ping -c 4 example.com`

- Ping with interval (seconds):

`ping -i 2 example.com`

- Ping with timeout:

`ping -W 5 example.com`

- Ping flood (requires root, DANGEROUS):

`sudo ping -f example.com`

- Ping with specific packet size:

`ping -s 1000 example.com`

- Ping quietly (only summary):

`ping -q -c 4 example.com`

- Ping with timestamp:

`ping -D example.com`

- Ping audible (beep on reply):

`ping -a example.com`

- Ping IPv6:

`ping6 example.com`

- Ping specific interface:

`ping -I eth0 example.com`

## Flags

- `-c count`: Stop after count packets
- `-i interval`: Seconds between packets
- `-W timeout`: Time to wait for response
- `-s size`: Packet size in bytes
- `-q`: Quiet output
- `-f`: Flood ping (root only)
- `-a`: Audible ping
- `-D`: Print timestamp
- `-I`: Network interface or source address
- `-4`: Force IPv4
- `-6`: Force IPv6

## Exit Codes

- `0`: Host is alive and responding
- `1`: No reply received
- `2`: Other error

## Output Meaning

- `64 bytes from...`: Successful reply
- `time=X ms`: Round-trip latency
- `ttl=X`: Time to live (hop limit)
- `Request timeout`: No response
- `Destination host unreachable`: Routing problem

## Common Errors

- "Name or service not known" - DNS failure or typo
- "Network is unreachable" - No route to host
- "Destination Host Unreachable" - Host down or filtered
- No response - Host may block ICMP (firewalled)

## Note

Some hosts block ICMP - no response doesn't always mean host is down. Try `nc` or `curl` as alternatives.
