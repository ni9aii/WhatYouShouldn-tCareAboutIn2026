//! Tests for the terminal RAII guard. These run without a real TTY: in CI the
//! guard cannot enter raw mode, so we only assert it fails cleanly and that
//! dropping a constructed guard does not panic.

use what_you_shouldnt_worry_about_in_2026::input::TerminalGuard;

#[test]
fn enter_returns_a_result_and_never_panics() {
    // In a non-TTY environment this is an Err; on a real terminal an Ok.
    // Either way the call must not panic.
    let _ = TerminalGuard::enter();
}

#[test]
fn guard_drops_without_panic_when_entered() {
    // Only exercise Drop where entering succeeded (real terminal).
    if let Ok(guard) = TerminalGuard::enter() {
        drop(guard);
    }
}

#[test]
fn guard_type_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TerminalGuard>();
    // Compile-time check that the guard is droppable.
    let _: fn(TerminalGuard) = |g| drop(g);
}
