# Agent instructions

## Branching

Never create a branch that tracks `origin/dev`. `git checkout -b <branch> origin/dev`,
`git branch <branch> origin/dev` and `git switch -c <branch> origin/dev` all set the
upstream to `dev` silently, and a bare `git push` on such a branch pushes straight
into `dev` — unreviewed commits land on the main branch from one careless command.

Branch off `dev` with tracking disabled, and push with the branch named explicitly:

```
git switch -c <branch> --no-track origin/dev
git push -u origin <branch>
```

If a branch already tracks `dev`, drop the upstream before doing anything else:
`git branch --unset-upstream`.

## Changelog policy

Every branch that is opened as a pull request into `dev` must describe its diff
against `dev` in a changelog. No PR into `dev` is complete without it.

### Write for the reader, not for the author

The reader is a devops engineer or a developer who runs the node and integrates
with it. They did not write the code and will not read it. Describe the surface
they can observe, in plain language:

- CLI flags, environment variables, config file options, and their defaults
- deployment surface: Ansible roles and playbooks, Makefile targets, scripts,
  Docker images
- API surface: GraphQL schema and query behaviour, REST/RPC endpoints,
  request and response shapes, error codes
- storage surface: database and archive schema, migrations, what has to be run
  and in which order
- consensus and node behaviour at a high level: block production, attestations,
  finalization, protocol version, network compatibility
- contract interfaces: ABI, events, getters, versions

Say what changed and what the reader has to do about it — carry a setting over
by hand, run a migration, upgrade through an intermediate release, switch a
query to a new field. Name flags, fields, tables and options exactly as they
appear in the product.

Leave out internal refactors, private renames, test-only changes and
implementation detail. If nothing observable changed, there is nothing to write.

### Which file

- `CHANGELOG.md` in the repo root — release notes for the node and the network
  as a whole. Sections: `Breaking Changes`, `New / Improvements`, `Fixes`.
- `<crate>/CHANGELOG.md` — the detailed, crate-level record. Sections:
  `Breaking Changes`, `Added`, `Changed`, `Fixed`, `Removed`. Crates that keep
  one: `block-manager`, `bm-archive-processor`, `gql-server`, `migration-tool`.

A change to one of those crates goes into the crate changelog in detail, and,
if an operator of the network would notice it, gets one condensed line in the
root `CHANGELOG.md` as well.

### Versions are assigned late

Release numbers are fixed only when an RC branch is cut from `dev` and frozen.
While work is landing on `dev`, nobody knows which release it will ship in.

While working on a branch or landing on `dev`:

- add entries under `## [Unreleased]` at the top of the changelog, directly
  above the newest released version; create that section if it is missing
- do not invent a version heading and do not bump `version` in any `Cargo.toml`
- add to the existing groups under `## [Unreleased]` rather than starting a
  second copy of them

At RC freeze, before merging into `main`, a human — not an agent:

- picks the real version numbers and bumps them in every affected `Cargo.toml`,
  including `package.version` in the root workspace manifest
- renames `## [Unreleased]` to `## [<version>] – <YYYY-MM-DD>` in each changelog
  that has entries

Sections of already released versions are history. Do not rewrite them, do not
move entries out of them, and do not append new entries to them.
