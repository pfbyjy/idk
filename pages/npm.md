# npm

> Node Package Manager. Manage JavaScript packages.
> See also: yarn, pnpm, npx
> Keywords: node, javascript, package, install, dependency, module

- Install dependencies from package.json:

`npm install`

- Install a package:

`npm install package-name`

- Install as dev dependency:

`npm install -D package-name`

- Install globally:

`npm install -g package-name`

- Install specific version:

`npm install package@1.2.3`

- Uninstall a package:

`npm uninstall package-name`

- Update all packages:

`npm update`

- Update specific package:

`npm update package-name`

- List installed packages:

`npm list`

- List top-level only:

`npm list --depth=0`

- List outdated packages:

`npm outdated`

- Run script from package.json:

`npm run script-name`

- Run tests:

`npm test`

- Start application:

`npm start`

- Initialize new package.json:

`npm init`

- Initialize with defaults:

`npm init -y`

- Search for packages:

`npm search keyword`

- View package info:

`npm info package-name`

- Clean cache:

`npm cache clean --force`

- Audit for vulnerabilities:

`npm audit`

- Fix vulnerabilities:

`npm audit fix`

## Flags

- `-g, --global`: Global installation
- `-D, --save-dev`: Save as dev dependency
- `-S, --save`: Save as dependency (default)
- `-E, --save-exact`: Save exact version
- `--production`: Skip dev dependencies
- `--legacy-peer-deps`: Ignore peer dependency conflicts
- `-y, --yes`: Accept defaults

## Common Scripts

- `npm start`: Start the application
- `npm test`: Run tests
- `npm run build`: Build for production
- `npm run dev`: Start development server
- `npm run lint`: Run linter

## Exit Codes

- `0`: Success
- `1`: Generic error

## Common Errors

- "EACCES permission denied" - Don't use sudo, fix npm permissions
- "peer dependency" - Use --legacy-peer-deps or fix versions
- "ENOENT" - package.json not found
- "ERESOLVE" - Dependency conflict, check versions
