# New Project

## Scripts

| Command | Action |
| yarn dev | Runs development version with Vite. |
| yarn build | Builds out to path specified in .env - server uses CARGO_MANIFEST dir as expected location. |
| yarn serve | Runs Vite preview. |
| yarn fmt | Runs prettier on JS, TS, and Svelte files. |
| yarn lint | Runs eslint on JS, TS, and Svelte files. |
| yarn lint:fix | Runs eslint with --fix on JS, TS, and Svelte files. |

## Notes

- Remember to change CORS settings; default is meant for debugging.
- Requires the .env file options set to connect to a PostgreSQL.
- 