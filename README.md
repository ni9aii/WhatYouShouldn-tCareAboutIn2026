# What You Shouldn't Care About in 2026

A satirical terminal game about the things you should not care about in 2026.

## Status

MVP-1 vertical slice is in development. It currently includes the shared game
state, the first elevator interaction, deterministic English oracle output, and
a terminal-safe `ratatui`/`crossterm` shell. Audio is not required.

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

In-game text is English through MVP-2. Russian localization is planned for
MVP-3.
