use vornix_banner::{Banner, BuiltinFont, Style, rgb, RUST_ORANGE};
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut banner_default = Banner::new("Vornix Rust 3D").bold();
    banner_default.display()?;

    thread::sleep(Duration::from_secs(1));

    let mut banner_custom = Banner::new("Vornix Rust 3D")
        .with_builtin_font(BuiltinFont::Larry3d)
        .with_style(
            Style::new()
                .fg(RUST_ORANGE)
                .bg(rgb(30, 30, 30))
                .bold()
        )
        .centered(true)
        .padding_top(2);

    banner_custom.display()?;

    Ok(())
}
//cargo run --example main --features presets
