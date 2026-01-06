# jq

> Command-line JSON processor. Essential for API work.
> See also: yq, fx, gron
> Keywords: json, parse, filter, query, transform, api

- Pretty print JSON:

`cat file.json | jq .`

- Get a field:

`echo '{"name":"foo"}' | jq '.name'`

- Get raw string (no quotes):

`echo '{"name":"foo"}' | jq -r '.name'`

- Get array element:

`echo '[1,2,3]' | jq '.[0]'`

- Get all array elements:

`echo '[1,2,3]' | jq '.[]'`

- Get nested field:

`jq '.data.items[0].name' file.json`

- Filter array:

`jq '.[] | select(.active == true)' file.json`

- Map over array:

`jq '[.[] | .name]' file.json`

- Get multiple fields:

`jq '{name: .name, id: .id}' file.json`

- Get array length:

`jq 'length' file.json`

- Get keys:

`jq 'keys' file.json`

- Compact output (no pretty print):

`jq -c '.' file.json`

- Combine files:

`jq -s '.' file1.json file2.json`

- Create JSON from arguments:

`jq -n --arg name "foo" '{"name": $name}'`

- Filter by field value:

`jq '.[] | select(.type == "error")' file.json`

- Sort array:

`jq 'sort_by(.name)' file.json`

- Unique values:

`jq 'unique' file.json`

- Transform API response:

`curl -s api.example.com | jq '.data[] | {id, name}'`

## Common Patterns

- `.field`: Get field
- `.[]`: Iterate array
- `.[0]`: First element
- `.field[]`: Iterate nested array
- `select(condition)`: Filter
- `map(expr)`: Transform each element
- `length`: Array/string length
- `keys`: Object keys
- `has("field")`: Check field exists
- `type`: Get value type
- `@base64`: Encode as base64
- `@uri`: URL encode

## Flags

- `-r, --raw-output`: Output raw strings
- `-c, --compact-output`: Compact output
- `-s, --slurp`: Read entire input into array
- `-n, --null-input`: Don't read input
- `--arg name value`: Set variable
- `-e, --exit-status`: Set exit code based on output
- `-S, --sort-keys`: Sort object keys

## Exit Codes

- `0`: Success (with -e: output was not false/null)
- `1`: Last output was false or null (with -e)
- `2`: Usage error
- `3`: Compile error
- `4`: Parse error
- `5`: Type error

## Common Errors

- "Cannot index null" - Field doesn't exist, use `?` operator
- "Cannot iterate over null" - Use `// []` for default empty array
- Quotes in output - Use -r for raw strings
