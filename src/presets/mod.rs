// --- src/presets/mod.rs ---
//! Módulo para banners o animaciones predefinidos.
//! Requiere la feature `presets`.

// Exportar los módulos de presets solo si la feature está activa
#[cfg(feature = "presets")]
pub mod ferris;

// #[cfg(feature = "presets")]
// pub mod titles; // Ejemplo si añadimos presets FIGlet