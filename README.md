# New Project

This is a base template for my current ideal web stack.

### Server Core Tech

Rust, Rocket, SQLx, PostgreSQL

### Client Core Tech

Svelte, TailwindCSS, ESLint, Prettier, Vite

## Scripts

| Command | Action |
|---------|--------|
| yarn dev | Runs development version with Vite. |
| yarn build | Builds out to path specified in .env - server uses CARGO_MANIFEST dir as expected location. |
| yarn serve | Runs Vite preview. |
| yarn fmt | Runs prettier on JS, TS, and Svelte files. |
| yarn lint | Runs eslint on JS, TS, and Svelte files. |
| yarn lint:fix | Runs eslint with --fix on JS, TS, and Svelte files. |

## Notes

- Has unsecure CORS settings meant for debugging.
- Tailwind JIT is enabled by default.
- Requires environment options set to connect to a PostgreSQL.
- Expects Rust to be on the nightly compiler.
- No test library as I don't have a preference client-side, and Rust is built-in.
- Expects migrations with `sqlx migrate`.
- Uses my preferred VSCode settings file.
- Uses Dependabot, Semantic Pull Request, and Release Drafter actions.
- License can be removed - it's just for personal ease-of-use.
