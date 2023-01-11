# Template: Rocket + Svelte SPA

## Backend

- [Rocket](https://rocket.rs)
- [PostgreSQL](https://www.postgresql.org)

## Frontend

- [Svelte](https://svelte.dev)
- [TypeScript](https://www.typescriptlang.org)
- [Tailwind CSS](https://tailwindcss.com)
- [Vite](https://vitejs.dev/) + [Vitest](https://vitest.dev/)
- [svelte-navigator](https://github.com/mefechoel/svelte-navigator) *(client-side routing)*

*Note: This template is built & tested against the latest Rocket release candidate (rc2 at the time of writing) and Rust nightly 1.66+.*

## Server Notes

- Built-in fairings for CORS (Cross-Origin Resource Sharing) and CSP (Content Security Policy).
- Built-in fairing for caching static files; caches JS, CSS, WEBP (images), and WOFF2 (fonts) files by default.
- **Includes a PostgreSQL fairing, which is disabled by default.** Add this fairing in `main.rs` if you wish to use it.

## Client Notes

- Contains lazy route handlers for loading routes asyncronously.

## GitHub Action Notes

- Runs (client) tests, eslint, tsc, and prettier on PRs.
- Runs dependabot weekly. You can manually run `combine` to squish all dependabot PRs into one PR.
- Server tests/formatting are not run on PR (yet).

## Usage

1. Clone the repository: `git clone https://github.com/robertwayne/template-rocket-svelte-spa`
2. Change `.env.TEMPLATE` to `.env` and set your Postgres credentials *(if not using defaults)*.
3. Run the server with `cargo run` from inside the `/server` directory.
4. You should build the client initially with `npm run build` from inside the `/client` directory. After that, you can use `npm run dev` for hot reloading.
5. If you're serving from the server, visit `http://localhost:8000` in your browser. If you're using npm dev, visit `http://localhost:5173` in your browser.

## Client Scripts

| Command | Action |
|---------|--------|
| npm run dev | Runs the Vite dev server. |
| npm run build | Builds out to path specified in .env - server uses CARGO_MANIFEST dir as expected location. |
| npm run serve | Runs Vite preview. |
| npm run fmt | Runs `prettier` on JS, TS, and Svelte files. |
| npm run lint | Runs `eslint` on JS, TS, and Svelte files. |
| npm run lint:fix | Runs `eslint` with --fix on JS, TS, and Svelte files. |
| npm run tsc | Runs `tsc --noEmit` on TS files. |
| npm run tsc:watch | Runs `tsc --noEmit --watch` on TS files. |
| npm run test | Runs `vitest` once. |
| npm run test:watch | Runs `vitest` in watch mode. |

## Misc Scripts

| Command | Action |
|---------|--------|
| ./update.sh | Updates the dependencies of both the client and server projects. |
