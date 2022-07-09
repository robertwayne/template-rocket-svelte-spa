# Template: Rocket + Svelte

This is a base template for my current web stack. On the backend, it uses [Rocket](https://rocket.rs) and [PostgreSQL](https://www.postgresql.org). On the frontend, it uses [Svelte](https://svelte.dev), [TypeScript](https://www.typescriptlang.org) and [Tailwind CSS](https://tailwindcss.com). Additional major dependencies include: ESLint, Prettier, Vite + Vitest, and svelte-navigator.

*Note: This template is built against the latest Rocket release candidate (rc2 at the time of writing) and Rust nightly 1.64+.*

## Server Notes

* Contains a base fairing for handling CORS requests.
* Contains a base fairing for an A++ Content Security Policy.
* Expects to connect to a PostgreSQL database. Remove this fairing in `main.rs` if you don't need a database.

## Client Notes

* Contains basic lazy route handlers for loading route content as it's needed.

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
| npm run test | Runs tests via vitest. |
