// --- src/style.rs ---
//! Define estilos (colores, atributos) para los banners usando `owo-colors`.

use owo_colors::{OwoColorize, Style as OwoStyle, DynColors, Rgb as OwoRgb};
use std::collections::HashSet;

/// Tipo alias para colores dinámicos de `owo-colors` (incluye estándar, bright, RGB).
pub type Color = DynColors;
/// Tipo alias para colores RGB de `owo-colors`.
pub type Rgb = OwoRgb;

// --- ELIMINADA la re-exportación de owo_colors::colors ---

// --- COLORES PREDEFINIDOS PERSONALIZADOS ---
pub const RUST_ORANGE: Color = DynColors::Rgb(222, 100, 40);
pub const FERRIS_BODY: Color = DynColors::Rgb(220, 90, 35);
pub const FERRIS_CLAWS: Color = DynColors::Rgb(180, 70, 25);
pub const DEFAULT_TEXT_COLOR: Color = DynColors::Rgb(64, 190, 197);

/// Atributos de texto soportados.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attribute { Bold, Italic, Underline }

/// Define el estilo visual de un banner.
#[derive(Debug, Clone, Default)]
pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub attributes: HashSet<Attribute>,
    owo_style: OwoStyle,
}

impl Style {
    pub fn new() -> Self {
        Self { owo_style: OwoStyle::new(), ..Default::default() }
    }
    // La firma espera 'Color' (DynColors)
    pub fn fg(mut self, color: Color) -> Self { self.foreground = Some(color); self.owo_style = self.owo_style.color(color); self }
    pub fn bg(mut self, color: Color) -> Self { self.background = Some(color); self.owo_style = self.owo_style.on_color(color); self }
    pub fn bold(mut self) -> Self { if self.attributes.insert(Attribute::Bold) { self.owo_style = self.owo_style.bold(); } self }
    pub fn italic(mut self) -> Self { if self.attributes.insert(Attribute::Italic) { self.owo_style = self.owo_style.italic(); } self }
    pub fn underline(mut self) -> Self { if self.attributes.insert(Attribute::Underline) { self.owo_style = self.owo_style.underline(); } self }
    pub(crate) fn apply(&self, text: &str) -> String { format!("{}", text.style(self.owo_style)) }
}

/// Helper para crear color RGB.
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    DynColors::Rgb(r, g, b)
}