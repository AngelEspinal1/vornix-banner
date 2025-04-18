// --- src/banner.rs ---
use crate::animation::Animation;
use crate::error::BannerError;
use crate::fonts::{load_font, BuiltinFont, FontSource};
use crate::style::Style;
// Quitamos FIGureOptions y HorizontalLayout de la importación
use std::path::Path;

/// Contenido del banner como vector de líneas.
pub type BannerContent = Vec<String>;

/// Opciones específicas para la generación de texto FIGlet.
/// **NOTA:** En `figlet-rs` 0.4, estas opciones tienen efecto limitado o nulo programáticamente.
/// El layout depende principalmente de la fuente `.flf`. El ancho no se aplica automáticamente.
#[derive(Debug, Clone, Default)]
pub struct FigletOptions {
    /// Ancho máximo deseado (informativo, no aplicado por `figlet-rs` 0.4).
    pub width: Option<u16>,
    // Eliminamos horizontal_layout ya que no se puede pasar a convert()
    // pub horizontal_layout: Option<HorizontalLayout>,
}

/// Representa un banner a generar a partir de texto o una animación predefinida.
/// Se configura con métodos builder y se muestra con `display()`.
#[derive(Debug, Clone)] // Ahora Clone funciona porque FIGfont no está en el cache
pub struct Banner {
    source: BannerSource,
    style: Style,
    centered: bool,
    padding_top: u16,
}

/// Contenido interno del banner.
#[derive(Debug, Clone)] // Clone funciona ahora
enum BannerSource {
    FigletText {
        text: String,
        font_source: FontSource,
        options: FigletOptions, // Mantenemos por si futuras versiones lo usan o para info
        /// Contenido generado cacheado (solo Vec<String>).
        generated_cache: Option<BannerContent>, // <-- Cambio: Solo BannerContent
    },
    AnimatedFrames {
        animation: Animation,
    },
}

impl Banner {
    /// Crea un nuevo banner FIGlet a partir de texto.
    pub fn new(text: &str) -> Self {
        Self {
            source: BannerSource::FigletText {
                text: text.to_string(),
                font_source: FontSource::default(),
                options: FigletOptions::default(),
                generated_cache: None, // Cache vacío inicialmente
            },
            style: Style::default(),
            centered: true,
            padding_top: 1,
        }
    }

    /// Crea un nuevo banner a partir de una animación predefinida.
    pub fn from_animation(animation: Animation) -> Self {
        Self {
            source: BannerSource::AnimatedFrames { animation },
            style: Style::default(),
            centered: true,
            padding_top: 1,
        }
    }

    // --- Métodos Builder ---

    /// Establece la fuente FIGlet a usar. Invalida la caché.
    pub fn with_font(mut self, font_source: FontSource) -> Self {
        if let BannerSource::FigletText { font_source: fs, generated_cache: cache, .. } = &mut self.source {
            *fs = font_source;
            *cache = None; // Resetear caché
        }
        self
    }

    /// Helper para usar fuente incrustada.
    pub fn with_builtin_font(self, builtin_font: BuiltinFont) -> Self {
        self.with_font(FontSource::Builtin(builtin_font))
    }

    /// Helper para usar fuente desde archivo.
    pub fn with_font_file(self, path: impl AsRef<Path>) -> Self {
        self.with_font(FontSource::File(path.as_ref().to_path_buf()))
    }

    /// Establece las opciones de diseño FIGlet (actualmente solo informativas). Invalida caché.
     pub fn with_figlet_options(mut self, options: FigletOptions) -> Self {
        if let BannerSource::FigletText { options: opts, generated_cache: cache, .. } = &mut self.source {
            *opts = options;
            // Aunque las opciones no afecten a convert(), reseteamos caché por consistencia
            *cache = None;
        }
        self
    }

    /// Establece el estilo general.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    // ... otros builders de estilo (sin cambios) ...
    pub fn with_foreground(mut self, color: crate::style::Color) -> Self {
        self.style = self.style.fg(color); self
    }
    pub fn with_background(mut self, color: crate::style::Color) -> Self {
        self.style = self.style.bg(color); self
    }
    pub fn bold(mut self) -> Self {
        self.style = self.style.bold(); self
    }
    pub fn italic(mut self) -> Self {
        self.style = self.style.italic(); self
    }
    pub fn underline(mut self) -> Self {
        self.style = self.style.underline(); self
    }
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered; self
    }
    pub fn padding_top(mut self, lines: u16) -> Self {
        self.padding_top = lines; self
    }

    /// Genera el contenido FIGlet si es necesario y lo cachea (solo `Vec<String>`).
    /// **Modificado:** Ya no devuelve referencias. Solo asegura que la caché esté poblada.
    fn ensure_generated_content(&mut self) -> Result<(), BannerError> {
        if let BannerSource::FigletText { text, font_source, /* options (no usado) */ generated_cache, .. } = &mut self.source {
            if generated_cache.is_none() {
                let fig_font = load_font(font_source)?;

                // --- CAMBIO: Usar convert() ---
                let figure = fig_font.convert(text)
                    .ok_or_else(|| BannerError::FigletGeneration(format!("No se pudo convertir texto FIGlet: '{}'", text)))?;

                let content_vec = figure.to_string().lines().map(String::from).collect();

                // --- CAMBIO: Cachear solo Vec<String> ---
                *generated_cache = Some(content_vec);
            }
        }
        Ok(())
    }


    /// Muestra el banner en la terminal.
    pub fn display(&mut self) -> Result<(), BannerError> {
        match &mut self.source {
            BannerSource::FigletText { .. } => {
                // --- CAMBIO: Separar préstamo mutable de inmutables ---
                // 1. Asegurar contenido generado (préstamo mutable termina aquí)
                self.ensure_generated_content()?;

                // 2. Obtener referencias inmutables para renderizar
                //    (volvemos a hacer match, ahora sobre &self implícitamente)
                if let BannerSource::FigletText { generated_cache: Some(content_vec), .. } = &self.source {
                     crate::renderer::display_static_content(
                         content_vec,       // Préstamo inmutable del cache
                         &self.style,       // Préstamo inmutable
                         self.centered,     // Copia (bool es Copy)
                         self.padding_top,  // Copia (u16 es Copy)
                     )
                } else {
                     // Esto solo ocurriría si ensure_generated_content falla lógicamente
                     Err(BannerError::Internal("Contenido generado FIGlet inesperadamente ausente después de ensure".to_string()))
                }
            }
            BannerSource::AnimatedFrames { animation } => {
                 // Para la animación, podemos tomar prestados los campos necesarios
                 // animation es Clone, así que podríamos clonarlo si fuera necesario, pero no hace falta
                 crate::renderer::display_animated(
                     animation,         // Préstamo inmutable
                     &self.style,       // Préstamo inmutable
                     self.centered,     // Copia
                     self.padding_top,  // Copia
                 )
            }
        }
    }
}