---
applyTo: ".github/workflows/cd.yml,.github/workflows/release.yml"
---

# Release review instructions

- Release automation uses three chained components: git-cliff for changelogs, release-plz for crate publishing and git tags, and cargo-dist for binary builds plus GitHub Releases.
- `cd.yml` runs release-plz on push to main. It creates or updates a release PR and publishes to crates.io when a version bump lands. Uses crates.io Trusted Publishing with OIDC and no `CARGO_REGISTRY_TOKEN`.
- `release.yml` should be regenerated with `dist generate --ci github` after changing `dist-workspace.toml`. It is triggered by tag pushes and builds cross-platform binaries for the `fmp-agent` CLI.
- The release-plz job must use `RELEASE_PLZ_TOKEN` so release PR branch pushes trigger normal CI workflows.
- `git_release_enable = false` in `release-plz.toml` because cargo-dist creates GitHub Releases instead.
- `cliff.toml` controls changelog format. Changes to commit groups or skip patterns should be reviewed for consistency with Conventional Commits.
