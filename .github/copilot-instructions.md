# Copilot review guidance

- This project is an unofficial Financial Modeling Prep client. Do not imply endorsement, sponsorship, or affiliation in generated docs, CLI text, or errors.
- CLI output must remain JSON. Success responses are the raw FMP JSON payload on one stdout line; failures from `main.rs` use `{ ok: false, error: { kind, message } }` on stderr.
- Keep `FMP_API_KEY`, API key values, `.env` contents, and account-specific data out of logs, errors, tests, and docs.
- Library-only consumers depend with `default-features = false`; preserve the no-default-features library build when changing feature gates.
- New endpoints must be confirmed in `docs/api-inventory.md` before becoming CLI commands, except intentionally denied endpoints that exist to verify structured subscription-error handling.
- CLI commands follow a two-level grouped structure: `fmp-agent <group> <subcommand>` (e.g., `fmp-agent market quote AAPL`, `fmp-agent company profile AAPL`). Flat single-token commands are rejected. Group modules live under `src/cli/groups/<name>.rs`, each with a `Cmd` enum and a `dispatch()` function. New endpoints belong in the appropriate group module. Four top-level aliases (`quote`, `historical`, `profile`, `earnings`) plus `search` and `schema` are the only accepted single-token commands.
- Endpoint additions should use `Endpoint` constants plus the existing shape helpers in `src/client.rs` and `src/cli/dispatch.rs`; avoid bespoke per-endpoint client wrappers.
- Tests should use `httpmock` rather than the live FMP API.
- Do not flag async test attributes unless the test body has no `.await` and no async-only setup.
