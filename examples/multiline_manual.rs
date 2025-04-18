// --- examples/multiline_manual.rs ---
use vornix_banner::{rgb, Banner, BuiltinFont, Style};
use std::io;

fn main() -> io::Result<()> {
    println!("--- Ejemplo: Efecto Multi-Línea Manual ---");

    let font = BuiltinFont::Block; // Fuente grande para ver el efecto
    let style = Style::new().fg(rgb(64, 190, 197)).bold();

    println!("\nMostrando banner en dos partes:");

    // Parte 1
    let mut banner1 = Banner::new("VORNIX").bold()
        .with_builtin_font(font)
        .with_style(style.clone()) // Clonar estilo si se reutiliza
        .padding_top(1)
        .centered(true);
    banner1.display()?;

    // Parte 2
    let mut banner2 = Banner::new("SERVICE").bold()
        .with_builtin_font(font)
        .with_style(style) // Usar el estilo
        .padding_top(0) // Sin padding extra
        .centered(true);
    banner2.display()?;

    println!("\nBanner multi-línea mostrado.");
    Ok(())
}