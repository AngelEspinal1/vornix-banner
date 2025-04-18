// --- src/utils.rs --- (¡Archivo Nuevo!)
//! Utilidades internas, como el cálculo preciso del ancho de texto.

use unicode_width::UnicodeWidthStr;
use regex::Regex;
use once_cell::sync::Lazy;

// Regex para encontrar códigos de escape ANSI. Compilado una sola vez.
// Fuente: https://github.com/chalk/ansi-regex/blob/main/index.js adaptado a Rust
static ANSI_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"[\u001B\u009B][\[\]()#;?]*((([a-zA-Z\d;]*[a-zA-Z\d]?)\u0007)|((\d{1,4}(?:;\d{0,4})*)?[\dA-PR-TZcf-ntqry=><~]))"
    ).unwrap()
});

/// Calcula el ancho visible de un string en la terminal.
///
/// Esta función:
/// 1. Elimina los códigos de escape ANSI usando una regex.
/// 2. Calcula el ancho usando `unicode-width`, que maneja correctamente
///    caracteres de ancho completo (CJK), emojis, etc.
///
/// # Argumentos
/// * `text` - El string cuyo ancho se va a medir. Puede contener códigos ANSI.
///
/// # Returns
/// El ancho visible en número de columnas de la terminal.
pub(crate) fn get_line_width_accurate(text: &str) -> u16 {
    // Eliminar códigos ANSI
    let cleaned_text = ANSI_REGEX.replace_all(text, "");
    // Calcular ancho Unicode
    UnicodeWidthStr::width(cleaned_text.as_ref()) as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use owo_colors::OwoColorize;

    #[test]
    fn test_width_no_ansi_ascii() {
        assert_eq!(get_line_width_accurate("hello"), 5);
    }

    #[test]
    fn test_width_no_ansi_unicode() {
        assert_eq!(get_line_width_accurate("你好"), 4); // Cada CJK char ocupa 2 columnas
        assert_eq!(get_line_width_accurate("🦀"), 2); // Emoji suele ocupar 2
        assert_eq!(get_line_width_accurate("hello你好🦀"), 5 + 4 + 2);
    }

    #[test]
    fn test_width_with_ansi() {
        let styled_text = "hello".red().bold().to_string();
        // El texto estilizado contiene códigos ANSI: "\u{1b}[1;31mhello\u{1b}[0m"
        assert_eq!(get_line_width_accurate(&styled_text), 5); // Debería ignorar ANSI

        let complex_styled = format!("你好 {}", "world".blue().on_green());
        assert_eq!(get_line_width_accurate(&complex_styled), 4 + 1 + 5); // "你好 world"
    }

     #[test]
    fn test_width_empty_string() {
        assert_eq!(get_line_width_accurate(""), 0);
        assert_eq!(get_line_width_accurate(&"".red().to_string()), 0);
    }
}