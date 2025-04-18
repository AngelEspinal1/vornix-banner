// --- src/lib.rs ---
// ... (documentación y otras partes) ...

// --- Módulos ---
mod banner;
pub mod style; // <--- ¡AÑADIR 'pub' AQUÍ!
mod animation;
mod renderer;
mod error;
mod fonts;
mod utils;

// --- Presets (Condicional) ---
#[cfg(feature = "presets")]
pub mod presets;

// --- Re-exportaciones Públicas ---
// Ya no necesitamos re-exportar TODO desde style aquí si el módulo es público,
// el usuario puede hacer `use vornix_banner::style::Style;` etc.
// Pero mantener algunas re-exportaciones clave en la raíz puede ser conveniente.
// Decide qué quieres que esté en `vornix_banner::` directamente y qué en `vornix_banner::style::`.

// Opción 1: Mantener re-exportaciones clave + módulo público (flexible)
pub use banner::{Banner, FigletOptions};
pub use style::{Color, Style, Attribute, Rgb, DEFAULT_TEXT_COLOR, RUST_ORANGE, rgb}; // Re-exportar tipos/helpers comunes de style
// Los colores específicos (Red, Blue...) se accederán vía ::style::Red o use ::style::Red
pub use animation::{Animation, Frame};
pub use error::BannerError;
pub use fonts::{FontSource, BuiltinFont};

// Opción 2: Solo hacer el módulo público y requerir `::style::` para todo lo de style
// pub use banner::{Banner, FigletOptions};
// pub use animation::{Animation, Frame};
// pub use error::BannerError;
// pub use fonts::{FontSource, BuiltinFont};
// // El usuario tendría que usar vornix_banner::style::Style, vornix_banner::style::Color, etc.

// Opción 3: Hacer el módulo público Y re-exportar todo con glob (como antes)
// pub use banner::{Banner, FigletOptions};
// pub use style::*; // <-- Esto re-exportaría TODO, incluyendo Red, Blue, etc. a la raíz
// pub use animation::{Animation, Frame};
// pub use error::BannerError;
// pub use fonts::{FontSource, BuiltinFont};

// --- Recomendación: Usar Opción 1 o 3 ---
// Opción 1 da un buen balance. Opción 3 es máxima conveniencia pero puede contaminar el namespace.
// Vamos a mantener la Opción 1 por ahora.