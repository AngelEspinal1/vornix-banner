// --- src/fonts.rs ---
// (Añadir docs y mejorar manejo de error por defecto)
//! Manejo de fuentes FIGlet (.flf), incluyendo carga desde archivos
//! y fuentes estándar incrustadas (detrás de la feature `standard_fonts`).

use figlet_rs::FIGfont;
use crate::error::BannerError;
use std::path::PathBuf;
use std::fs;

/// Identifica las fuentes FIGlet estándar que pueden ser incrustadas.
/// Requiere activar la feature `standard_fonts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinFont {
    Standard,
    Slant,
    Small,
    Block,
    Larry3d,
    Lean
    // futura expansión: Mini, Banner, etc.
}

/// Define el origen de la fuente FIGlet a usar.
#[derive(Debug, Clone)]
pub enum FontSource {
    /// Usar una fuente estándar incrustada.
    Builtin(BuiltinFont),
    /// Cargar la fuente desde un archivo `.flf` en la ruta especificada.
    File(PathBuf),
}

// --- Contenido de fuentes incrustadas (requiere feature) ---
#[cfg(feature = "standard_fonts")]
const STANDARD_FLF: &str = include_str!("../fonts/standard.flf");
#[cfg(feature = "standard_fonts")]
const SLANT_FLF: &str = include_str!("../fonts/slant.flf");
#[cfg(feature = "standard_fonts")]
const SMALL_FLF: &str = include_str!("../fonts/small.flf");
#[cfg(feature = "standard_fonts")]
const BLOCK_FLF: &str = include_str!("../fonts/block.flf");
#[cfg(feature = "standard_fonts")]
const LARRY3D_FLF: &str = include_str!("../fonts/larry3d.flf");
#[cfg(feature = "standard_fonts")]
const LEAN_FLF: &str = include_str!("../fonts/lean.flf");


/// Carga y parsea una fuente FIGlet desde el origen especificado.
///
/// # Errores
/// Devuelve `BannerError::FontLoad` si:
/// * Se intenta usar `BuiltinFont` sin la feature `standard_fonts` habilitada.
/// * El archivo especificado en `FontSource::File` no se puede leer.
/// * El contenido de la fuente no se puede parsear como una fuente FIGlet válida.
pub(crate) fn load_font(source: &FontSource) -> Result<FIGfont, BannerError> {
    match source {
        FontSource::Builtin(builtin) => {
            #[cfg(feature = "standard_fonts")]
            {
                let font_content = match builtin {
                    BuiltinFont::Standard => STANDARD_FLF,
                    BuiltinFont::Slant => SLANT_FLF,
                    BuiltinFont::Small => SMALL_FLF,
                    BuiltinFont::Block => BLOCK_FLF,
                    BuiltinFont::Larry3d => LARRY3D_FLF,
                    BuiltinFont::Lean => LEAN_FLF
                };
                FIGfont::from_content(font_content)
                    .map_err(|e| BannerError::FontLoad(format!("Error al parsear fuente incrustada {:?}: {}", builtin, e)))
            }
            #[cfg(not(feature = "standard_fonts"))]
            {
                // Error claro si se pide fuente builtin sin la feature
                Err(BannerError::FontLoad(format!(
                    "Se requiere fuente incrustada {:?}, pero la feature 'standard_fonts' no está habilitada.",
                    builtin
                )))
            }
        }
        FontSource::File(path) => {
            let content = fs::read_to_string(path)
                .map_err(|e| BannerError::FontLoad(format!("No se pudo leer archivo de fuente {:?}: {}", path, e)))?;
            FIGfont::from_content(&content)
                 .map_err(|e| BannerError::FontLoad(format!("Error al parsear fuente desde {:?}: {}", path, e)))
        }
    }
}


impl Default for FontSource {
    /// Proporciona una fuente por defecto (`BuiltinFont::Standard`).
    ///
    /// **Nota:** Si la feature `standard_fonts` no está habilitada, usar esta
    /// fuente por defecto resultará en un error `BannerError::FontLoad` cuando
    /// se intente cargarla (normalmente durante `Banner::display()`). En ese caso,
    /// se debe especificar explícitamente una fuente con `Banner::with_font_file()`.
    fn default() -> Self {
        FontSource::Builtin(BuiltinFont::Standard)
    }
}