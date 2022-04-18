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
| npm run dev | Runs development version with Vite. |
| npm run build | Builds out to path specified in .env - server uses CARGO_MANIFEST dir as expected location. |
| npm run serve | Runs Vite preview. |
| npm run fmt | Runs prettier on JS, TS, and Svelte files. |
| npm run lint | Runs eslint on JS, TS, and Svelte files. |
| npm run lint:fix | Runs eslint with --fix on JS, TS, and Svelte files. |
| npm run test | Runs jest w/ support for ESM and TypeScript. |

## Notes

- Has unsecure CORS settings meant for debugging.
- Requires environment options set to connect to a PostgreSQL.
- Expects Rust to be on the nightly compiler.
- Expects migrations with `sqlx migrate`.
- Uses a dependabot workflow.
