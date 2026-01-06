# dig

> DNS lookup utility. Query DNS servers.
> See also: nslookup, host, whois
> Keywords: dns, lookup, domain, nameserver, query, resolve

- Simple DNS lookup:

`dig example.com`

- Get just the IP (short answer):

`dig +short example.com`

- Query specific record type:

`dig example.com MX`

- Query all record types:

`dig example.com ANY`

- Use specific DNS server:

`dig @8.8.8.8 example.com`

- Reverse DNS lookup:

`dig -x 8.8.8.8`

- Trace DNS resolution path:

`dig +trace example.com`

- Query with no extra info:

`dig +noall +answer example.com`

- Get TTL values:

`dig +ttlunits example.com`

- Query TXT records (SPF, DKIM, etc.):

`dig example.com TXT`

- Query nameservers:

`dig example.com NS`

- Query SOA record:

`dig example.com SOA`

- Check DNSSEC:

`dig +dnssec example.com`

- Batch queries from file:

`dig -f domains.txt`

## Common Record Types

- `A`: IPv4 address
- `AAAA`: IPv6 address
- `MX`: Mail server
- `NS`: Nameserver
- `TXT`: Text (SPF, DKIM, verification)
- `CNAME`: Canonical name (alias)
- `SOA`: Start of authority
- `PTR`: Pointer (reverse DNS)
- `SRV`: Service location

## Flags

- `+short`: Brief output
- `+trace`: Trace delegation path
- `+noall +answer`: Only answer section
- `+ttlunits`: Show TTL in human units
- `+dnssec`: Request DNSSEC records
- `-x`: Reverse lookup
- `-f`: Read queries from file
- `@server`: Use specific DNS server

## Public DNS Servers

- `8.8.8.8`: Google
- `1.1.1.1`: Cloudflare
- `9.9.9.9`: Quad9
- `208.67.222.222`: OpenDNS

## Exit Codes

- `0`: Success
- `1`: Usage error
- `8`: Couldn't open batch file
- `9`: No reply from server
- `10`: Internal error

## Common Errors

- "connection timed out" - DNS server unreachable
- "NXDOMAIN" - Domain doesn't exist
- "SERVFAIL" - DNS server error
