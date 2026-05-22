---
description: Prepare and publish a rusty-fmp release
---

# Make Release

Prepare a `rusty-fmp` crate release and publish the `fmp-agent` binary artifacts through the tagged release workflow. Use one confirmation prompt before changing files, committing, tagging, or pushing.

## Step 0: Check for Version Override

If the user provides `$ARGUMENTS` as a specific version, such as `0.4.0`, skip auto-detection and use that exact version as `<new-version>`. Jump to Step 3 after reading the current state needed for the summary.

Validate that `$ARGUMENTS` contains only one semantic version string. If it is missing, continue with Step 1.

## Step 1: Read Current State

Read the current package version, the latest release tag, and the commits since that tag.

```bash
# Get current version
grep '^version' Cargo.toml | head -1 | sed 's/.*= "\(.*\)"/\1/'

# Get last tag
git tag --list --sort=-version:refname | head -1

# Get commits since last tag
git log $(git tag --list --sort=-version:refname | head -1)..HEAD --oneline --no-merges
```

If there is no previous tag, use all reachable commits for release detection and say so in the summary.

## Step 2: Detect Bump Type

Parse the conventional commit subjects and bodies since the last tag. Use this command as the source input for detection.

```bash
git log --format="%s %b" $(git tag --list --sort=-version:refname | head -1)..HEAD
```

Detect the bump type with these rules:

- If any commit body contains `BREAKING CHANGE`, or any subject has `!` before `:`, choose a minor bump because this project is still `0.x` and breaking changes bump `0.3.0` to `0.4.0`, not `1.0.0`.
- Else if any commit subject starts with `feat`, choose a minor bump.
- Else if any commit subject starts with `fix`, choose a patch bump.
- Else abort with a clear message: no conventional release commits were found. Tell the user to provide an explicit version with `$ARGUMENTS`, add a conventional commit, or stop the release.

Display the current version, bump type, new version, and the commits that triggered the bump. If both `feat` and `fix` commits exist, show the `feat` commits as the reason because minor outranks patch.

## Step 3: Confirm Before Acting

Show one release summary before making any changes:

- Current version
- Last tag
- Selected bump type or `$ARGUMENTS` override
- New version
- Triggering commits
- Files that will change: `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`
- Git actions that will happen: release commit, signed annotated tag, branch push, tag push

Ask exactly once: `Proceed? [y/N]`. Abort unless the answer is exactly `y` or `Y`.

## Step 4: Bump Version

Bump the crate version and check the project before generating release notes. If `cargo set-version` is unavailable, tell the user to install it with `cargo install cargo-edit --locked`, then stop.

```bash
cargo set-version <new-version>
cargo check
```

## Step 5: Generate Changelog

Regenerate the entire changelog with git-cliff, using the new version as the release tag. The `cliff.toml` file already filters conventional commits and skips release-preparation commits.

```bash
git cliff --tag <new-version> -o CHANGELOG.md
```

Inspect `CHANGELOG.md` for obvious release-note mistakes before committing. Do not edit `cliff.toml` as part of this command.

## Step 6: Commit Release Files

Stage only the release files, then create a signed release-preparation commit.

```bash
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -s -S -m "chore(release): prepare <new-version>"
```

If `Cargo.lock` did not change, omit it from the staged files rather than staging unrelated changes.

## Step 7: Tag and Push

Create a signed annotated tag, push the branch, then push the tag to trigger the release workflow.

```bash
git tag -s -a <new-version> -m "Release <new-version>"
git push
git push origin <new-version>
```

The tag push triggers cargo-dist, which builds release artifacts and runs the crates.io OIDC publish job.

## Rules

- **0.x semver**: Until the project reaches 1.0, `BREAKING CHANGE` bumps minor (`0.3.0` to `0.4.0`), not major. Only use `$ARGUMENTS` to override to `1.0.0` manually when ready.
- **What happens after push**: The tag triggers `release.yml` in CI. cargo-dist builds multi-platform binaries. The `publish` job authenticates via OIDC and runs `cargo publish`, with no API token stored anywhere.
- **Prerequisite**: crates.io Trusted Publisher must be configured for this crate with owner `major`, repo `fmp-rs`, workflow file `release.yml`. Only needed once.
- **Post-migration cleanup**: Remove `RELEASE_PLZ_TOKEN` from GitHub repo secrets after first successful OIDC publish.
- **Dry run**: Use `$ARGUMENTS` with a specific version to override auto-detection if needed.
- Never add a second confirmation prompt. Step 3 is the only confirmation gate.
- Never push if any command before Step 7 fails.
