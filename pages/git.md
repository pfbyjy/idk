# git

> Distributed version control system. Track changes, collaborate, manage code history.
> See also: gh, git-log, git-diff, git-branch
> Keywords: version control, vcs, commit, branch, merge, push, pull, repository, clone

- Clone a repository:

`git clone https://github.com/user/repo.git`

- Check status of working directory:

`git status`

- Stage files for commit:

`git add file.txt`

- Stage all changes:

`git add -A`

- Commit staged changes:

`git commit -m "commit message"`

- Push to remote:

`git push origin branch-name`

- Pull latest changes:

`git pull origin branch-name`

- Create and switch to new branch:

`git checkout -b new-branch`

- Switch branches:

`git checkout branch-name`

- View commit history:

`git log --oneline -n 10`

- View changes (unstaged):

`git diff`

- View changes (staged):

`git diff --staged`

- Stash changes temporarily:

`git stash`

- Apply stashed changes:

`git stash pop`

- Reset file to last commit:

`git checkout -- file.txt`

- Undo last commit (keep changes):

`git reset --soft HEAD~1`

- Fetch without merging:

`git fetch origin`

## Flags

- `-m`: Commit message
- `-b`: Create new branch
- `-A, --all`: Stage all changes
- `--oneline`: Compact log format
- `--staged`: Show staged changes in diff
- `--soft`: Reset keeping changes staged
- `--hard`: Reset discarding all changes (DANGEROUS)
- `-f, --force`: Force operation (DANGEROUS for push)

## Exit Codes

- `0`: Success
- `1`: Generic error
- `128`: Fatal error (bad repo, permission denied, etc.)

## Common Errors

- "fatal: not a git repository" - Run git init or cd to repo directory
- "Your branch is behind" - Run git pull before pushing
- "CONFLICT" - Resolve merge conflicts manually, then git add and commit
- "Permission denied (publickey)" - Set up SSH keys or use HTTPS with token
- "refusing to merge unrelated histories" - Use --allow-unrelated-histories flag
