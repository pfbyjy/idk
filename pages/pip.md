# pip

> Python package installer.
> See also: pipenv, poetry, conda
> Keywords: python, package, install, dependency, module, pypi

- Install a package:

`pip install package-name`

- Install specific version:

`pip install package==1.2.3`

- Install from requirements file:

`pip install -r requirements.txt`

- Install in user directory:

`pip install --user package-name`

- Install in editable/development mode:

`pip install -e .`

- Upgrade a package:

`pip install --upgrade package-name`

- Upgrade pip itself:

`pip install --upgrade pip`

- Uninstall a package:

`pip uninstall package-name`

- List installed packages:

`pip list`

- List outdated packages:

`pip list --outdated`

- Show package info:

`pip show package-name`

- Search for packages (deprecated, use pypi.org):

`pip search keyword`

- Freeze installed packages:

`pip freeze > requirements.txt`

- Download without installing:

`pip download package-name`

- Install from git:

`pip install git+https://github.com/user/repo.git`

- Check for dependency issues:

`pip check`

- Use specific Python version:

`python3.9 -m pip install package`

## Flags

- `--user`: Install to user directory
- `-e, --editable`: Editable install (for development)
- `-r, --requirement`: Install from requirements file
- `-U, --upgrade`: Upgrade package
- `--no-cache-dir`: Disable cache
- `--force-reinstall`: Reinstall all packages
- `-t, --target`: Install to specific directory
- `-q, --quiet`: Less output
- `-v, --verbose`: More output

## Exit Codes

- `0`: Success
- `1`: Generic error
- `2`: Dependency error

## Common Errors

- "Could not find version" - Package doesn't exist or typo
- "Permission denied" - Use --user or virtual environment
- "No matching distribution" - Wrong Python version or OS
- "Requirement already satisfied" - Already installed
- Always use virtual environments to avoid conflicts!

## Virtual Environment Pattern

```bash
python -m venv venv
source venv/bin/activate  # Linux/Mac
pip install -r requirements.txt
```
