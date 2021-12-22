# New Project

This is a base template for my current web stack.

## Server Core Tech

Rust, Rocket, SQLx, PostgreSQL

The server builds against the Rocket 0.5 release candidate.

## Client Core Tech

Svelte, svelte-navigator, TailwindCSS v3, Jest, ESLint, Prettier, Vite

## Scripts

| Command | Action |
|---------|--------|
| yarn dev | Runs development version with Vite. |
| yarn build | Builds out to path specified in .env - server uses CARGO_MANIFEST dir as expected location. |
| yarn serve | Runs Vite preview. |
| yarn fmt | Runs prettier on JS, TS, and Svelte files. |
| yarn lint | Runs eslint on JS, TS, and Svelte files. |
| yarn lint:fix | Runs eslint with --fix on JS, TS, and Svelte files. |
| yarn test | Runs jest w/ support for ESM and TypeScript. |

## Notes

- Has unsecure CORS settings meant for debugging.
- Requires environment options set to connect to a PostgreSQL.
- Expects Rust to be on the nightly compiler.
- Expects migrations with `sqlx migrate`.
- Uses my preferred VSCode settings file. You should remove this.
- Uses Dependabot, Semantic Pull Request, and Release Drafter actions.
- License can be removed - it's just for personal ease-of-use.
