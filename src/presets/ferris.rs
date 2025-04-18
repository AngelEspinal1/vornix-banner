// --- src/presets/ferris.rs ---
//! Animación predefinida de Ferris el cangrejo caminando.
//! Requiere la feature `presets`.

use crate::{
    animation::{Animation, Frame},
    banner::Banner, // BannerContent no es estrictamente necesario aquí si usamos Frame
    style::{Style, FERRIS_BODY},
};

// --- Frames de la Animación ---
// ¡Eliminamos las definiciones duplicadas/placeholders!

const FERRIS_FRAME_1: &[&str] = &[
    r#"       .--."#, r#"     / \  `."#, r#"    |  ."#, r#"    \  ;___/"#,
    r#"     '--'"#, r#"    /____\"#, r#"   (( L L "#, r#"    `-----'"#,
];
const FERRIS_FRAME_2: &[&str] = &[
    r#"       .--."#, r#"     / \  `."#, r#"    |  ."#, r#"    \  ;___/"#,
    r#"     '--'"#, r#"    /____\"#, r#"   (( / L "#, r#"    `---->' "#,
];
const FERRIS_FRAME_3: &[&str] = &[ // Igual que frame 1 para ciclo A-B-A-C (o A-B-C-B si frame 4 es diferente)
    r#"       .--."#, r#"     / \  `."#, r#"    |  ."#, r#"    \  ;___/"#,
    r#"     '--'"#, r#"    /____\"#, r#"   (( L L "#, r#"    `-----'"#,
];
const FERRIS_FRAME_4: &[&str] = &[
    r#"       .--."#, r#"     / \  `."#, r#"    |  ."#, r#"    \  ;___/"#,
    r#"     '--'"#, r#"    /____\"#, r#"   (( L \ "#, r#"    `<----'"#,
];


/// Helper para convertir `&[&str]` a `Frame`.
fn make_frame(lines: &[&str]) -> Frame {
    lines.iter().map(|s| s.to_string()).collect()
}

/// Crea un `Banner` configurado con la animación de Ferris caminando.
///
/// La animación se repite indefinidamente y se muestra centrada con un
/// estilo naranja por defecto.
#[cfg(feature = "presets")]
pub fn walking_ferris() -> Banner {
    let frames = vec![
        make_frame(FERRIS_FRAME_1),
        make_frame(FERRIS_FRAME_2),
        make_frame(FERRIS_FRAME_3),
        make_frame(FERRIS_FRAME_4),
    ];

    let animation = Animation::new(frames, 180) // Delay en ms
        .repeat(None); // Repetir indefinidamente

    // Crear el Banner usando from_animation
    Banner::from_animation(animation)
        .with_style(Style::new().fg(FERRIS_BODY))
        .centered(true)
        .padding_top(1)
}