# What You Shouldn't Care About in 2026

A satirical terminal game about the things you should not care about in 2026.

## Status

MVP-1 is implemented. It includes a segment abstraction (`Segment` trait), a
terminal-safe `ratatui`/`crossterm` shell with an onboarding screen and a
segment-selection menu, elevator/radio/mirror segments, a seeded `GameState`
for reproducible runs, and a classified Oracle with 30+ deterministic English
verdict templates. Audio is not required.

## License

This project is distributed under `GPL-3.0-only`. See [LICENSE](LICENSE).

The project is open source. Distributed derivative works must comply with the
GPLv3 source-disclosure and licensing requirements. Third-party assets and
licenses will be documented separately before they are bundled.

## Development

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked -- -D warnings
cargo build --locked --release
cargo run --release
```

The playable MVP-1 flow shows an onboarding screen, then a segment menu where
you pick which mini-game to play. Each segment presents a decision and updates
the shared player profile; your progress (completed segments) is kept while you
play. After completing three segments the Oracle verdict unlocks and can be
requested from the menu with `v`.

Keyboard controls:
- Title / onboarding: `Enter` proceeds. `q` or `Esc` quits.
- Segment menu: number keys `1`/`2`/`3` (or `↑`/`↓` + `Enter`) choose a segment.
  `v` shows the Oracle verdict once unlocked. `Esc`/`q` quits. Completed segments
  are marked `[done]`.
- Floor input (elevator): type digits, `Enter` confirms, `Backspace` erases,
  `Esc` returns to the menu, `q` quits.
- Yes/No prompts: `y`/`Y` selects yes, `n`/`N` selects no, then `Enter`
  confirms the choice. `Enter` alone confirms no. `Esc` returns to the menu,
  `q` quits. Selections are not applied until you press `Enter`.
- After the verdict: `r` replays, `Enter`/`q`/`Esc` quits.

`Esc` returns to the menu without losing the current run. `q` always quits the
game. Runs are reproducible: `GameState::with_seed` fixes the RNG for tests and
replays.

In-game text is English through MVP-2. Russian localization is planned for
MVP-3.
