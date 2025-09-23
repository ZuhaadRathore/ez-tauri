# Release Readiness TODOs

## Backend data layer

- [x] Replace placeholder SQL in `src-tauri/src/handlers/users.rs` (select/insert/update/delete/auth queries) with valid statements using bind parameters.
- [x] Ensure password storage uses bcrypt (matches seeded hashes) instead of `format!("hashed_{}", password)`.
- [x] Fill `src-tauri/src/handlers/logs.rs` queries with proper placeholders and filtering logic; add pagination guards.
- [ ] Add integration/unit tests for database handlers touching real pool or mocks.

## System & security hardening

- [x] Implement safe notification plumbing via `tauri-plugin-notification` rather than `Ok(format!(...))` in `src-tauri/src/handlers/system.rs`.
- [x] Gate `execute_command` behind an allow-list and capture stderr/stdout securely.
- [x] Audit filesystem handlers for path traversal and add tests covering restricted paths.

## Logging configuration alignment

- [x] Standardize env var names between `.env.example` and `src-tauri/src/logging/mod.rs` (e.g. `LOG_CONSOLE_ENABLED`, `LOG_FILE_ENABLED`, `LOG_DIRECTORY`).
- [x] Verify `EnvFilter` setup honors log level strings and add documentation for supported values.
- [x] Provide runtime toggle to reload config without restart or update README to clarify restart requirement.

## Testing & CI

- [ ] Update `playwright.config.ts` webServer command to launch the Tauri dev process (or dedicated start script) and confirm port alignment.
- [ ] Add backend-focused tests (e.g. `cargo test` cases for migrations, handlers) so CI covers Rust logic.
- [ ] Confirm CI matrices install required system dependencies and caches (Rust + Node) efficiently; add coverage thresholds if desired.

## Release pipeline

- [ ] Replace `__VERSION__` placeholder in `.github/workflows/release.yml` with workflow input or derive from package/Cargo versions.
- [ ] Add changelog generation or release notes template step.
- [ ] Define restrictive CSP in `src-tauri/tauri.conf.json` before shipping (no `null`).

## Documentation & onboarding

- [ ] Either create the promised `/docs` content or adjust README to remove the "coming soon" note.
- [ ] Replace emoji headings in `README.md` with ASCII-friendly icons or ensure proper encoding.
- [ ] Document database bootstrap expectations (Docker requirement, seeds, migrations) and add troubleshooting section for Postgres/Redis.

## Verification checklist

- [ ] Run `npm run lint`, `npm run test`, `npm run test:e2e`, and `cargo test` after fixes.
- [ ] Perform `npm run tauri:build` on Windows/macOS/Linux targets (CI or local) to verify packaging.
- [ ] Smoke-test installers from release artifacts before announcing GA.
