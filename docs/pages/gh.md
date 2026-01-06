# gh

> GitHub CLI - work with GitHub from the command line.
> See also: git
> Keywords: github, pull request, issue, pr, repo, clone, fork

- Clone a repository:

`gh repo clone owner/repo`

- Create a new repository:

`gh repo create my-repo --public`

- Fork a repository:

`gh repo fork owner/repo`

- Create a pull request:

`gh pr create --title "Title" --body "Description"`

- Create PR interactively:

`gh pr create`

- List open pull requests:

`gh pr list`

- View pull request:

`gh pr view 123`

- Checkout a pull request locally:

`gh pr checkout 123`

- Merge a pull request:

`gh pr merge 123`

- Create an issue:

`gh issue create --title "Bug" --body "Description"`

- List issues:

`gh issue list`

- View issue:

`gh issue view 123`

- View repo in browser:

`gh repo view --web`

- View PR in browser:

`gh pr view 123 --web`

- Check CI status:

`gh pr checks`

- List releases:

`gh release list`

- Download release assets:

`gh release download v1.0.0`

- Authenticate:

`gh auth login`

## Flags

- `--web, -w`: Open in browser
- `--title, -t`: Title for PR/issue
- `--body, -b`: Body/description
- `--repo, -R`: Specify repository
- `--json`: Output as JSON
- `--jq`: Filter JSON output

## Exit Codes

- `0`: Success
- `1`: Error
- `4`: Authentication required

## Common Errors

- "gh auth login" - Need to authenticate first
- "not a git repository" - Must be in a git repo
- "no pull requests match" - PR doesn't exist or wrong number
