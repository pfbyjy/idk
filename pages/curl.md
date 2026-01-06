# curl

> Transfer data to/from a server using URLs. The Swiss Army knife of HTTP requests.
> See also: wget, httpie
> Keywords: http, https, request, download, upload, api, rest, post, get, web

- Download a file to stdout:

`curl https://example.com`

- Download and save with remote filename:

`curl -O https://example.com/file.tar.gz`

- Download and save to specific file:

`curl -o output.txt https://example.com/data`

- Make a POST request with JSON data:

`curl -X POST -H "Content-Type: application/json" -d '{"key": "value"}' https://api.example.com/endpoint`

- Make a POST request with form data:

`curl -X POST -d "name=value&other=data" https://example.com/form`

- Follow redirects:

`curl -L https://example.com/redirecting-url`

- Include response headers in output:

`curl -i https://example.com`

- Send with authentication (basic auth):

`curl -u username:password https://api.example.com/secure`

- Send with bearer token:

`curl -H "Authorization: Bearer TOKEN" https://api.example.com/secure`

- Download silently (no progress bar):

`curl -s https://example.com/data`

- Set request timeout:

`curl --connect-timeout 5 --max-time 10 https://example.com`

- Send custom headers:

`curl -H "X-Custom-Header: value" https://example.com`

## Flags

- `-X, --request`: HTTP method (GET, POST, PUT, DELETE, PATCH)
- `-H, --header`: Add custom header
- `-d, --data`: Send data in request body
- `-o, --output`: Write output to file
- `-O, --remote-name`: Save with remote filename
- `-L, --location`: Follow redirects
- `-i, --include`: Include response headers
- `-s, --silent`: Silent mode, no progress
- `-v, --verbose`: Verbose output for debugging
- `-f, --fail`: Fail silently on HTTP errors (no output)
- `-k, --insecure`: Allow insecure SSL connections

## Exit Codes

- `0`: Success
- `6`: Could not resolve host
- `7`: Failed to connect to host
- `22`: HTTP error (with -f flag)
- `28`: Operation timeout
- `35`: SSL connect error
- `56`: Failure receiving network data

## Common Errors

- "Could not resolve host" - Check URL spelling, DNS, or internet connection
- "Connection refused" - Server not running or wrong port
- "SSL certificate problem" - Use -k to bypass (only for testing!) or fix cert chain
- Empty response with no error - Server returned empty body, check with -i to see headers
