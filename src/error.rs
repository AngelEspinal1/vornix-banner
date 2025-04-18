// --- src/error.rs ---
//! Define los tipos de error internos para la librería `term_banner`
//! y la conversión a `std::io::Error`.

use std::io;
use thiserror::Error;
// No importamos ErrorKind de crossterm

/// Errores internos que pueden ocurrir durante la creación o visualización de banners.
#[derive(Error, Debug)]
pub enum BannerError {
    /// Error de entrada/salida estándar (puede originarse en IO, crossterm, terminal_size).
    #[error("Error de entrada/salida o Terminal: {0}")]
    Io(#[from] io::Error),

    // Variante Crossterm ELIMINADA

    /// Error al cargar o parsear un archivo de fuente FIGlet (.flf).
    #[error("Error al cargar o parsear la fuente FIGlet: {0}")]
    FontLoad(String),

    /// Error durante la generación del arte ASCII a partir de texto con `figlet-rs`.
    #[error("Error al generar el arte FIGlet: {0}")]
    FigletGeneration(String),

    /// La configuración proporcionada para una animación manual es inválida.
    #[error("Configuración de animación inválida: {0}")]
    InvalidAnimation(String),

    /// No se pudo determinar el tamaño de la terminal.
    #[error("No se pudo determinar el tamaño de la terminal: {0}")]
    TerminalSize(io::Error),

    /// Error interno inesperado en la librería.
    #[error("Error interno de la librería: {0}")]
    Internal(String),
}

// --- Conversión a std::io::Error ---
/// Permite convertir `BannerError` en `std::io::Error`.
/// Esto es útil para que el operador `?` funcione en funciones que devuelven `io::Result<()>`,
/// como `Banner::display()`. Pierde algo de especificidad del error original.
impl From<BannerError> for io::Error {
    fn from(err: BannerError) -> Self {
        let kind = match &err { // Tomamos prestado err para poder usar to_string después
            BannerError::Io(io_err) => io_err.kind(),
            BannerError::TerminalSize(io_err) => io_err.kind(),
            BannerError::FontLoad(_) => io::ErrorKind::InvalidData,
            BannerError::FigletGeneration(_) => io::ErrorKind::InvalidData,
            BannerError::InvalidAnimation(_) => io::ErrorKind::InvalidInput,
            BannerError::Internal(_) => io::ErrorKind::Other,
            // Si añadiéramos un BannerError::Crossterm(e), mapearíamos 'e' aquí
        };
        // Usamos el mensaje generado por `thiserror` (`err.to_string()`)
        io::Error::new(kind, err.to_string())
    }
}