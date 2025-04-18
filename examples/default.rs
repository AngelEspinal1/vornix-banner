// --- examples/01_basic_default.rs ---
use vornix_banner::{Banner, BannerError}; // Solo lo básico

fn main() -> Result<(), BannerError> {
    println!("--- Ejemplo: Banner Básico con Defaults ---");
    println!("(Fuente: Standard, Estilo: Cyan-ish Negrita, Centrado)");

    // Banner::new() usa los defaults que establecimos
    let mut banner = Banner::new("Default Banner").bold(); // Ahora display está implícito

    // Banner::new() ahora aplica el estilo y fuente default
    // No necesitamos .with_style ni .with_builtin_font para el default

    banner.display()?; // Mostrar

    println!("\nBanner mostrado con éxito.");
    Ok(())
}