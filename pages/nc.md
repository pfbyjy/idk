# nc

> Netcat - networking Swiss Army knife. TCP/UDP connections and listeners.
> See also: curl, telnet, socat
> Keywords: network, tcp, udp, port, listen, connect, socket, test

- Test if port is open:

`nc -zv host 80`

- Scan port range:

`nc -zv host 20-25`

- Connect to a port:

`nc host 80`

- Listen on a port:

`nc -l 8080`

- Listen and keep listening after disconnect:

`nc -lk 8080`

- Send file:

`nc host 1234 < file.txt`

- Receive file:

`nc -l 1234 > received.txt`

- Simple chat (one listens, one connects):

`nc -l 1234`

- UDP mode:

`nc -u host 53`

- Set connection timeout:

`nc -w 5 host 80`

- HTTP request:

`echo -e "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n" | nc example.com 80`

- Port forwarding (simple proxy):

`nc -l 8080 | nc host 80`

- Verbose output:

`nc -v host 80`

## Flags

- `-z`: Zero-I/O mode (scanning)
- `-v`: Verbose
- `-l`: Listen mode
- `-k`: Keep listening after disconnect
- `-u`: UDP mode
- `-w`: Timeout in seconds
- `-p`: Local port
- `-n`: No DNS resolution
- `-4`: IPv4 only
- `-6`: IPv6 only

## Exit Codes

- `0`: Success
- `1`: Error

## Common Uses

- **Port scanning**: `nc -zv host 1-1000`
- **Banner grabbing**: `echo "" | nc -v host 22`
- **Test HTTP**: `echo -e "GET /\r\n" | nc host 80`
- **Check if listening**: `nc -zv localhost 3000`
- **Quick file transfer**: listener receives, sender sends

## Common Errors

- "Connection refused" - Nothing listening on that port
- "Connection timed out" - Firewall or host unreachable
- "Address already in use" - Port already bound

## Variants

Different systems have different netcat versions:
- GNU netcat
- OpenBSD netcat (ncat)
- ncat (from nmap)

Flags may vary slightly between versions.
