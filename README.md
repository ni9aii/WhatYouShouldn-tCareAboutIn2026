# What You Shouldn't Care About in 2026

A satirical terminal game about the things you should not care about in 2026.

## Status

MVP-1 vertical slice is in development. It currently includes the shared game
state, elevator/radio/mirror interactions, deterministic English oracle output,
and a terminal-safe `ratatui`/`crossterm` shell. Audio is not required.

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

The playable MVP-1 flow waits for Enter (or `q` / `Esc`) on the title screen,
then runs the elevator, radio, and mirror segments in sequence. Each segment
presents a binary decision and updates the shared player profile. The elevator
also asks whether the player panics after choosing a floor. The Oracle verdict
is shown after all three segments are complete.

Keyboard controls:
- Title screen: `Enter` starts the session. `q` or `Esc` quits.
- Floor input: type digits, `Enter` confirms, `Backspace` erases, `Esc`
  returns to the menu, `q` quits.
- Yes/No prompts: `y`/`Y` selects yes, `n`/`N` selects no, then `Enter`
  confirms the choice. `Enter` alone confirms no. `Esc` returns to the menu,
  `q` quits. Selections are not applied until you press `Enter`.
- After the verdict: `r` replays, `Enter`/`q`/`Esc` quits.

`Esc` returns to the menu without losing the current run. `q` always quits the
game.

In-game text is English through MVP-2. Russian localization is planned for
MVP-3.
