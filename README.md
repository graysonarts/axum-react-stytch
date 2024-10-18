# axum-react-stytch stack

This is an opinionated stack that has Axum as a rust backend, react for the frontend, stytch for authentication, and types shared between rust and typescript using wasm.

To use this, `cargo generate https://github.com/graysonarts/axum-react-stytch`

I built this because I got tired of rewriting the same boilerplate all the time.

## Setting environment variables

Use the `.env.example` in `backend` and `frontend` to setup things correctly. Rust uses `dotenvy` and React uses `vite` to load the variables.
