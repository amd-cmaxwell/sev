# Plan: Per-crate GitHub Releases with Binding Artifacts

## Context

The release workflow creates per-crate GitHub Releases rather than a single monolithic release.
The goal is to align bindings and Rust crates under the same commit by:
1. Detecting which crates have version bumps since the last release
2. Building bindings and committing them
3. Tagging Go modules on the bindings commit
4. Creating per-crate GitHub Releases (tagged `crates/{crate-dir}/v{version}`) with only that crate's artifacts attached

## Trigger

The workflow uses `workflow_dispatch` (manual trigger) with no inputs. All version information is derived automatically from `Cargo.toml` files compared against the previous `v*` tag.

## Job flow

```
detect-changes
  ├── build-python-bindings  (if has-ffi-changes)
  ├── build-go-bindings      (if has-ffi-changes)
  └── build-js-bindings      (if has-ffi-changes)
        └── publish-bindings (if has-ffi-changes)
              ├── commits bindings to main
              └── tags Go modules
                    └── release-rust-crates (if has-crate-changes)
                          creates per-crate GitHub Releases
```

## detect-changes job

Outputs four values consumed by downstream jobs:

| Output | Format | Description |
|--------|--------|-------------|
| `ffi-libs` | space-separated, underscored | Changed FFI crate names (e.g. `core trig`) |
| `has-ffi-changes` | `true`/`false` | Gates binding build/publish jobs |
| `crate-versions` | JSON object | Map of changed library crates to versions (e.g. `{"core":"0.2.0","trig":"0.1.1"}`) |
| `has-crate-changes` | `true`/`false` | Gates the release job |

Both detection loops compare the `version` field in each `Cargo.toml` against the same file at the previous `v*` tag.

## publish-bindings job

- Downloads Go, Python, and JS binding artifacts
- Commits bindings to `main`
- Commit message enumerates modified crates from the `crate-versions` JSON: `chore: Commit FFI bindings for core v0.2.0, trig v0.1.1`
- Tags Go modules using `REPO_NAME_US`, `PKG_SUFFIX_US`, and the FFI crate version

## release-rust-crates job

Replaces the former `tag-rust-crates` + `release` jobs.

1. Checks out with full history (`fetch-depth: 0`)
2. Downloads wheel artifacts (`wheel-*` pattern, merged) and npm tarballs (`npm-tarballs`)
3. Iterates over `crate-versions` JSON via `jq`
4. For each modified crate:
   - Creates tag `crates/{crate-dir}/v{version}`
   - Matches artifacts using template-friendly patterns derived from `REPO_NAME` and `PKG_SUFFIX`:
     - Wheels: `*${REPO_NAME_US}_${crate_us}${PKG_SUFFIX_US}*`
     - NPM tarballs: `*${REPO_NAME}-${crate_dir}${PKG_SUFFIX}*`
   - Creates a GitHub Release via `gh release create` with matched artifacts attached
5. Pushes all tags after the loop

### Template-friendly naming

Artifact matching avoids hardcoded project names. Patterns are constructed at runtime from `REPO_NAME` and `PKG_SUFFIX` environment variables (set via repository `vars`), following the same convention as the Makefile:
- Kebab-case: `${REPO_NAME}-${crate_dir}${PKG_SUFFIX}`
- Underscore: `${REPO_NAME_US}_${crate_us}${PKG_SUFFIX_US}`

## Files modified

- `.github/workflows/release.yml`

## Verification

- Review the generated YAML for syntax correctness
- `gh release create` uses `GH_TOKEN: ${{ github.token }}` which is available with `contents: write` permission
- Artifact name patterns match those produced by the build workflows (confirmed against `build-python-bindings.yml` and `build-js-bindings.yml`)
