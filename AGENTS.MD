# AGENTS.MD

## Project

- Repository: `ni9aii/WhatYouShouldntWorryAboutIn2026`
- Local checkout path: repository root
- Default branch: `main`
- Project notes: keep local development notes outside this repository
- GitHub repository is the source of truth for project history.

## Working principles

- Work to completion: do not stop at a stub, plan, or unverified command.
- Prefer small, reviewable changes over broad speculative refactors.
- Make one regular commit per logical component or feature.
- Keep the repository clean: do not commit generated files, local configuration,
  credentials, secrets, machine-specific paths, or real infrastructure details.
- Use English for repository documentation, code comments, commit messages,
  public project artifacts, and all in-game text. Russian-language in-game text
  is deferred to MVP-3.
- Update documentation whenever behavior, workflow, public interfaces, or
  project conventions change.

## Git workflow

- Never commit directly to `main`.
- Start every change from an up-to-date `main` branch:

  ```bash
  git switch main
  git pull --rebase origin main
  ```

- Create a branch before editing. Use one of these prefixes:
  - `feat/<short-description>` — new functionality
  - `fix/<short-description>` — bug fix
  - `docs/<short-description>` — documentation-only change
  - `test/<short-description>` — tests or test infrastructure
  - `chore/<short-description>` — maintenance and tooling
  - `refactor/<short-description>` — behavior-preserving restructuring
  - `release/<version>` — release preparation
- Use lowercase kebab-case descriptions in branch names.
- Commit messages use Conventional Commits, for example:
  - `feat: add ...`
  - `fix: correct ...`
  - `docs: document ...`
  - `test: cover ...`
  - `chore: update ...`
- Push feature branches and open a pull request into `main`.
- Do not force-push or rewrite history without explicit user approval.
- Do not merge upstream branches or unrelated changes into this repository.
- Before handing work over, inspect `git status`, the diff, and the commit list.

## AutoDev workflow

Use the following sequence for non-trivial work:

1. Review — inspect the repository and, when useful, run independent code,
   security, architecture, and DevOps reviews in parallel.
2. Aggregate — consolidate findings into one plan with two sections:
   `Do Now` and `Defer to Next Phase`.
3. Execute — implement all applicable `Do Now` items, splitting complex work
   into small tasks and committing each logical change.
4. Verify — run the relevant local tests, linters, formatters, builds, and
   security checks.
5. CI — integrate the checks into GitHub Actions and verify the resulting run.
6. Release — only when explicitly requested; ask before creating a release or
   pushing to a protected/default branch.

For AutoDev artifacts, use the project notes directory and this layout:

```text
<external-notes-directory>/
├── reviews/<YYYYMMDD_HHMMSS>/
│   ├── code-review.md
│   ├── security-review.md
│   ├── architecture-review.md
│   └── devops-review.md
├── plans/<YYYYMMDD_HHMMSS>-plan.md
└── ci-reports/<YYYYMMDD_HHMMSS>-ci-status.md
```

Review findings should use severity headings such as:

```markdown
### [CRITICAL] Short title
File: `path/to/file`
Line: 42
Description: Concrete, reproducible explanation.
```

## Quality gates

Every new behavior must have tests appropriate to the project. Tests must be
integrated into CI rather than existing only as local commands.

Before opening a pull request:

- Run the project's formatter.
- Run the project's linter or static analysis.
- Run the complete test suite.
- Run a release or production-oriented build when applicable.
- Review CI workflow changes for least-privilege permissions, locked or
  reproducible dependencies, and job timeouts.
- Check for secrets, credentials, accidental local paths, debug output, and
  unresolved merge markers.
- Update README and other relevant documentation.

Do not report a task as complete until the implementation has been exercised
and the relevant results are available from real tool output.

## Repository structure

- Keep source code, tests, documentation, CI configuration, and scripts in
  conventional locations for the selected technology.
- Add a README before the project becomes difficult to understand without
  repository history.
- Keep generated output and local tooling state out of version control.
- If the project uses Rust, do not add Python tooling to the repository; use
  Rust or shell scripts instead.

## Notes and cross-repository links

- Store durable project plans, reviews, status reports, and CI reports under
  the project directory in `dev-notes`.
- Use Obsidian wikilinks without the `.md` suffix.
- Project README notes should use full vault-relative links when linking to
  files in other project directories.
- Keep project notes factual: include dates, commands, statuses, counts, and
  verified results; do not replace evidence with assumptions.

## Safety

- Never commit secrets or real infrastructure identifiers.
- Preserve user changes. Inspect local branches and uncommitted work before
  recovery or cleanup operations.
- Avoid destructive Git commands such as `git reset --hard`, history rewrites,
  branch deletion, or force-push unless the user explicitly authorizes them.
- Ask before destructive changes, production changes, releases, or operations
  that affect protected branches.
