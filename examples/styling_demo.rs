// --- examples/styling_demo.rs ---
use vornix_banner::{
    Banner, BuiltinFont, Style,
    RUST_ORANGE, rgb // Helpers/Constantes
};
use std::io;

const DEMO_FONT: BuiltinFont = BuiltinFont::Standard;

fn main() -> io::Result<()> {
    println!("--- Demostración de Colores y Estilos (RGB/Constantes) ---");

    println!("\nEstilo: Rojo FG (RGB)");
    let mut banner_red_fg = Banner::new("Red FG (RGB)").bold()
        .with_builtin_font(DEMO_FONT)
        .with_style(Style::new().fg(rgb(200, 0, 0)).bold());
    banner_red_fg.display()?;

    println!("\nEstilo: Amarillo BG (RGB)");
     let mut banner_yellow_bg = Banner::new("Yellow BG (RGB)").bold()
        .with_builtin_font(DEMO_FONT)
        .with_style(Style::new().bg(rgb(255, 255, 0)).fg(rgb(0,0,0))); // FG negro para contraste
    banner_yellow_bg.display()?;

    println!("\nEstilo: Constante RUST_ORANGE");
    let mut banner_rust = Banner::new("Rust Orange").bold()
        .with_builtin_font(DEMO_FONT)
        .with_style(Style::new().fg(RUST_ORANGE).italic()); // Usar constante
    banner_rust.display()?;

    println!("\nEstilo: Default (Constante Interna)");
     let mut banner_default_style = Banner::new("Default Style").bold()
        .with_builtin_font(DEMO_FONT);
        // .with_style(Style::new().fg(DEFAULT_TEXT_COLOR).bold()) // Ya aplicado por new()
    banner_default_style.display()?;


    println!("\nEstilo: Combinado (Púrpura sobre Gris, Atributos)");
    let mut banner_combo = Banner::new("Combo Style")
        .with_builtin_font(DEMO_FONT)
        .with_style(
            Style::new()
                .fg(rgb(180, 100, 255)) // Púrpura
                .bg(rgb(50, 50, 50))    // Gris oscuro
                .bold()
                .underline()
        );
    banner_combo.display()?;

    println!("\n--- Fin de la demostración de estilos ---");
    Ok(())
}