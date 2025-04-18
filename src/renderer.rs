// --- src/renderer.rs ---
//! Módulo responsable de dibujar los banners (estáticos y animados) en la terminal
//! usando `crossterm`. Incluye manejo de modo raw, pantalla alternativa y limpieza.

use crate::animation::Animation;
use crate::banner::BannerContent;
use crate::error::BannerError;
use crate::style::Style;
use crate::utils::get_line_width_accurate; // Para cálculo preciso de ancho
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, style as crossterm_style,
    terminal::{self, ClearType},
};
use std::{io::{self, stdout, Stdout, Write}, panic::{self, PanicHookInfo}}; // Importar io para usarlo en get_terminal_width

use terminal_size::{Height, Width}; // Importar Width y Height

// --- Terminal Guard ---
/// Gestiona el estado de la terminal (modo raw, pantalla alternativa, cursor)
/// y asegura su restauración incluso en caso de panic.
struct TerminalGuard {
    // Usar PanicHookInfo si está disponible y no da problemas, si no, mantener PanicInfo por compatibilidad temporal
    original_panic_hook: Option<Box<dyn Fn(&PanicHookInfo<'_>) + Send + Sync + 'static>>,
    // Alternativa:
    // original_panic_hook: Option<Box<dyn Fn(&PanicHookInfo<'_>) + Send + Sync + 'static>>,
}

impl TerminalGuard {
    /// Entra en modo raw, cambia a pantalla alternativa y oculta el cursor.
    /// Instala un hook de panic para asegurar la restauración.
    fn new() -> Result<Self, BannerError> {
        // 1. Guardar el hook original
        let original_hook = panic::take_hook();

        // 2. Establecer un hook simple que SOLO restaura la terminal
        panic::set_hook(Box::new(move |_panic_info: &PanicHookInfo<'_>| {
            // No llamamos al hook original aquí, solo restauramos
            let _ = Self::restore_terminal_static();
        }));

        let mut stdout = stdout();
        terminal::enable_raw_mode()?;
        execute!(stdout, cursor::Hide, terminal::EnterAlternateScreen)?;

        // 3. Guardar el hook original en la struct
        Ok(Self { original_panic_hook: Some(original_hook) })
    }

    /// Restaura la terminal a su estado original (estático para el hook de panic).
    fn restore_terminal_static() -> Result<(), BannerError> {
        let mut stdout = stdout();
        execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Restaura la terminal (método de instancia que también restaura el hook).
    fn restore_terminal(&mut self) -> Result<(), BannerError> {
        if let Some(hook) = self.original_panic_hook.take() {
            panic::set_hook(hook);
        }
        Self::restore_terminal_static()
    }
}

impl Drop for TerminalGuard {
    /// Asegura la restauración de la terminal al salir del scope.
    fn drop(&mut self) {
        if let Err(e) = self.restore_terminal() {
            eprintln!("[term_banner] Advertencia: Error al restaurar la terminal: {}", e);
        }
    }
}


/// Renderiza contenido estático (ej. FIGlet generado) en la terminal.
pub(crate) fn display_static_content(
    content: &BannerContent,
    style: &Style,
    centered: bool,
    padding_top: u16,
) -> Result<(), BannerError> {
    let mut stdout = stdout();
    let terminal_width = get_terminal_width().unwrap_or(80); // Mantenemos fallback

    apply_vertical_padding(&mut stdout, padding_top)?;

    for line in content {
        let line_width = get_line_width_accurate(line);
        let padding = calculate_padding(terminal_width, line_width, centered);
        let styled_line = style.apply(line);

        queue!(
            stdout,
            crossterm_style::Print(" ".repeat(padding as usize)),
            crossterm_style::Print(&styled_line),
            crossterm_style::Print("\r\n")
        )?;
    }
    stdout.flush()?;
    Ok(())
}

/// Renderiza una animación de frames predefinidos.
pub(crate) fn display_animated(
    animation: &Animation,
    style: &Style,
    centered: bool,
    padding_top: u16,
) -> Result<(), BannerError> {
    if animation.frames.is_empty() { return Ok(()); }

    let mut _guard = TerminalGuard::new()?;
    let mut stdout = stdout();
    let terminal_width = get_terminal_width().unwrap_or(80); // Mantenemos fallback

    let mut current_repeat = 0;
    let mut frame_index = 0;

    loop {
        if poll(animation.frame_delay)? {
            match read()? {
                Event::Key(KeyEvent { code: KeyCode::Esc, .. }) |
                Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) |
                Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. }) => {
                    break;
                }
                _ => {}
            }
        }

        let frame_content = &animation.frames[frame_index];
        queue!( stdout, terminal::Clear(ClearType::Purge), cursor::MoveTo(0, 0) )?;
        apply_vertical_padding(&mut stdout, padding_top)?;

        // --- CORRECCIÓN AQUÍ ---
        // Obtener posición, manejar Result con '?', luego acceder al elemento .1 (row)
        let start_row = match cursor::position() {
             Ok((_, r)) => r, // Si Ok, obtener la fila (r)
             Err(_) => padding_top, // Si falla, usar un fallback razonable (ej: padding_top)
                                    // O podrías propagar el error: Err(e) => return Err(BannerError::Io(e.into()))?
                                    // Depende de cuán crítico sea obtener la posición exacta.
        };
        // Alternativa más corta si quieres propagar el error:
        // let start_row = cursor::position()?.1;


        for (i, line) in frame_content.iter().enumerate() {
            let line_width = get_line_width_accurate(line);
            let padding = calculate_padding(terminal_width, line_width, centered);
            let styled_line = style.apply(line);

            queue!(
                stdout,
                // Usar start_row calculado
                cursor::MoveTo(padding, start_row.saturating_add(i as u16)), // Usar saturating_add por si acaso
                crossterm_style::Print(&styled_line)
            )?;
        }
        stdout.flush()?;

        frame_index += 1;
        if frame_index >= animation.frames.len() {
            frame_index = 0;
            match animation.repeat {
                Some(count) => {
                    current_repeat += 1;
                    if current_repeat >= count { break; }
                }
                None => {}
            }
        }
    }
    Ok(())
}


// --- Funciones Helper ---

/// Obtiene el ancho actual de la terminal.
fn get_terminal_width() -> Result<u16, BannerError> {
    // --- CORRECCIÓN AQUÍ ---
    // 1. Llamar a terminal_size() y manejar el io::Result con '?'
    let size_option: Option<(Width, Height)> = Some(terminal_size::terminal_size().expect("REASON")); // Propaga BannerError::Io si falla

    // 2. Manejar el Option: si es None, devolver nuestro error específico; si es Some, extraer ancho.
    size_option
        .map(|(Width(w), _)| w) // Extraer ancho 'w' si es Some
        .ok_or_else(|| {        // Si es None, crear y devolver BannerError::TerminalSize
            BannerError::TerminalSize(io::Error::new(
                io::ErrorKind::Unsupported, // Usar 'Unsupported' como kind para indicar no-TTY o fallo similar
                "No es una TTY o no se pudo obtener tamaño",
            ))
        })
}


/// Calcula el padding izquierdo necesario para centrar el contenido.
fn calculate_padding(terminal_width: u16, content_width: u16, centered: bool) -> u16 {
    if centered && terminal_width > content_width {
        terminal_width.saturating_sub(content_width) / 2
    } else {
        0
    }
}

/// Aplica padding vertical superior imprimiendo N líneas nuevas.
fn apply_vertical_padding(stdout: &mut Stdout, padding: u16) -> Result<(), BannerError> {
    if padding > 0 {
        let padding_str = "\r\n".repeat(padding as usize);
        queue!(stdout, crossterm_style::Print(padding_str))?;
    }
    Ok(())
}