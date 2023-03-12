# Template: Rocket + Svelte SPA

## Backend

- __[Rocket](https://rocket.rs)__
- __[PostgreSQL](https://www.postgresql.org)__

## Frontend

- __[Svelte](https://svelte.dev)__
- __[svelte-navigator](https://github.com/mefechoel/svelte-navigator)__
- __[TypeScript](https://www.typescriptlang.org)__
- __[Tailwind CSS](https://tailwindcss.com)__
- __[Vite](https://vitejs.dev/)__ + __[Vitest](https://vitest.dev/)__

## Getting Started

- Clone the repository: `git clone
  https://github.com/robertwayne/template-rocket-svelte-spa`
- Change `.env.TEMPLATE` to `.env` and set your Postgres credentials _(if not
  using defaults)_.
- Build the client with `pnpm run build` from inside the `/client` directory.
   _Alternatively, you can use `pnpm run dev` to run the client with vite dev
   server._
- Run the server with `cargo run` from inside the `/server` directory.
  - If you're serving from Rocket, visit `http://127.0.0.1:3000`.
  - If you're serving from vite, visit `http://127.0.0.1:8000`.

## Client Notes

- Async, lazy loading route wrappers.
- Responsive navigation menu built-in.

## Server Notes

- Sets Cache Control headers for HTML, CSS, JS, WEBP, SVG, and WOFF2.
- Sets CORS and CSP headers.
- Includes a PostgreSQL fairing, which is disabled by default. Add this fairing
  in `main.rs` if you wish to use it.

## GitHub Action Notes

- Runs _(client)_ tests, eslint, tsc, and prettier on PRs.
- Runs dependabot weekly. You can manually run `combine` to squish all
  dependabot PRs into one PR.
- Server tests/formatting are not run on PR _(yet)_.

## Misc Scripts

| Command | Action |
|---------|--------|
| ./update.sh | Updates the dependencies of both the client and server projects. |

## Other Templates

- __[Axum + SolidJS](https://github.com/robertwayne/template-axum-solidjs-spa)__
