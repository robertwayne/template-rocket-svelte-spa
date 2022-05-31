# Template: Rocket + Svelte

This is a base template for my current web stack. On the backend, it uses [Rocket](https://rocket.rs) and [PostgreSQL](https://www.postgresql.org). On the frontend, it uses [Svelte](https://svelte.dev), [TypeScript](https://www.typescriptlang.org) and [Tailwind CSS](https://tailwindcss.com).

*Note: This template is built against the latest Rocket release candidate (rc2 at the time of writing) and Rust nightly 1.63+.*

## Client Core Tech

Svelte, svelte-navigator, TailwindCSS v3, Jest, ESLint, Prettier, Vite

## Usage

1. Clone the repository: `git clone https://github.com/robertwayne/template-rocket-svelte-spa`
2. Change `.env.TEMPLATE` to `.env` and set your Postgres credentials (if not using defaults).
3. Run the server with `cargo run` from inside the `/server` directory.
4. You should build the client initially with `npm run build` from inside the `/client` directory. After that, you can use `npm run dev` for hot reloading.
5. If you're serving from the server, visit `http://localhost:8000` in your browser. If you're using npm dev, visit `http://localhost:3000` in your browser.

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

- Requires environment options set to connect to a PostgreSQL. You can remove the fairing in `main.rs` to skip using Postgres.
- Expects Rust to be on the nightly compiler.
