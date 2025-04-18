// --- src/animation.rs ---
// (Sin cambios respecto a la versión anterior, pero añadir docs)

//! Define estructuras para animaciones basadas en frames predefinidos.

use crate::banner::BannerContent;
use std::time::Duration;

/// Representa un único frame de una animación ASCII (un conjunto de líneas).
pub type Frame = BannerContent;

/// Configuración para una animación de banner basada en frames.
#[derive(Debug, Clone)]
pub struct Animation {
    /// La secuencia de frames que componen la animación.
    pub frames: Vec<Frame>,
    /// El tiempo a esperar entre la visualización de cada frame.
    pub frame_delay: Duration,
    /// Cuántas veces repetir la animación completa. `None` significa bucle infinito.
    pub repeat: Option<usize>,
}

impl Animation {
    /// Crea una nueva configuración de animación.
    ///
    /// # Argumentos
    /// * `frames` - Los frames (vectores de strings) de la animación.
    /// * `frame_delay_ms` - El retardo entre frames en milisegundos.
    pub fn new(frames: Vec<Frame>, frame_delay_ms: u64) -> Self {
        Self {
            frames,
            frame_delay: Duration::from_millis(frame_delay_ms),
            repeat: Some(1), // Por defecto, reproducir la secuencia una vez
        }
    }

    /// Establece el número de veces que la animación debe repetirse.
    /// `None` indica un bucle infinito.
    pub fn repeat(mut self, count: Option<usize>) -> Self {
        self.repeat = count;
        self
    }

    /// Establece el retardo entre frames usando `Duration`.
    pub fn delay(mut self, duration: Duration) -> Self {
        self.frame_delay = duration;
        self
    }

    /// Establece el retardo entre frames en milisegundos.
    pub fn delay_ms(mut self, ms: u64) -> Self {
        self.frame_delay = Duration::from_millis(ms);
        self
    }
}