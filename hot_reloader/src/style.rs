//! ANSI colour helpers for the live-coding logs.
//!
//! Colours are emitted only when stdout is an interactive terminal, so
//! redirected output stays plain and parseable (the test suite relies on
//! this).

/// Wrap `text` in an ANSI SGR code, but only when stdout is an interactive
/// terminal (redirected output stays plain and parseable).
pub fn paint(code: &str, text: &str) -> String {
    use std::io::IsTerminal;
    if std::io::stdout().is_terminal() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

/// Dim (grey) text.
pub fn paint_dim(text: &str) -> String {
    paint("2", text)
}

/// Cyan text (addresses, byte dumps).
pub fn paint_cyan(text: &str) -> String {
    paint("36", text)
}

/// Bright cyan text (counts, headers).
pub fn paint_bright_cyan(text: &str) -> String {
    paint("1;96", text)
}

/// Green text (success values).
pub fn paint_green(text: &str) -> String {
    paint("32", text)
}

/// Bright green text (success headlines).
pub fn paint_bright_green(text: &str) -> String {
    paint("1;92", text)
}

/// Yellow text (warnings / fallbacks).
pub fn paint_yellow(text: &str) -> String {
    paint("33", text)
}

/// Bright yellow text (decision headlines).
pub fn paint_bright_yellow(text: &str) -> String {
    paint("1;93", text)
}

/// Red text (errors).
pub fn paint_red(text: &str) -> String {
    paint("31", text)
}

/// Bright red text (changed status).
pub fn paint_bright_red(text: &str) -> String {
    paint("1;91", text)
}

/// Magenta text (section accents).
pub fn paint_magenta(text: &str) -> String {
    paint("35", text)
}

/// `[hot]` prefix used on the main live-coding log lines.
pub fn hot_prefix() -> String {
    paint_bright_cyan("[hot]")
}

/// A labelled horizontal rule used as a section banner, e.g.
/// `── source analysis ─────────────────────────────────────────────`.
pub fn rule(title: &str) -> String {
    const WIDTH: usize = 72;
    let label = format!("── {title} ");
    let fill = WIDTH.saturating_sub(label.chars().count());
    paint_bright_cyan(&format!("{label}{}", "─".repeat(fill)))
}
