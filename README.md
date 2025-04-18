# Vornix Banner 🦀
<p align="center">
  <img src="assets/logo.svg" width="200" alt="Vornix Banner Logo"/>
</p>


**Una librería de Rust para generar y mostrar banners de texto ASCII estilo FIGlet en la terminal, con colores, fuentes personalizables y animación opcional.**

## Descripción

**Vornix Banner** es parte de la iniciativa Vornix para traer herramientas robustas y eficientes al ecosistema Rust, inspirados por nuestra experiencia en el mundo Java. Esta librería facilita la creación de banners llamativos para tus aplicaciones de consola, similar a lo visto en frameworks como Spring Boot, pero con la potencia y seguridad de Rust.

## Características Clave

- Generación FIGlet: Convierte texto a arte ASCII.
- Fuentes Flexibles: Usa fuentes estándar incrustadas o carga archivos `.flf` externos.
- Estilos Avanzados: Colores RGB y atributos (negrita, itálica, etc.).
- Presets de Configuración: Métodos rápidos para banners comunes (Default, Warning, Success).
- Animación: Soporte para animaciones frame-a-frame manuales.
- Layout: Opciones de centrado y padding.
- Renderizado Robusto: Limpieza y restauración en caso de panic.
- Unicode & ANSI: Cálculo correcto del ancho de caracteres.

## Instalación

Añade `vornix-banner` a tu `Cargo.toml`:

```toml
[dependencies]
vornix-banner = { version = "0.0.1", features = ["presets"] }
```

## Ejemplo de uso

Copia el siguiente código en tu `main.rs` para probar la librería:

```rust
use vornix_banner::{Banner, BuiltinFont, Style, rgb, RUST_ORANGE};
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut banner_default = Banner::new("Vornix Rust 3D").bold();
    banner_default.display()?;

    thread::sleep(Duration::from_secs(1));

    let mut banner_custom = Banner::new("Vornix Rust 3D")
        .with_builtin_font(BuiltinFont::Larry3d)
        .with_style(
            Style::new()
                .fg(RUST_ORANGE)
                .bg(rgb(30, 30, 30))
                .bold()
        )
        .centered(true)
        .padding_top(2);

    banner_custom.display()?;

    Ok(())
}
```

### Ejecutar el ejemplo

```bash
cargo run --example main --features presets
```

## Fuentes integradas

La librería incluye las siguientes fuentes builtin:

- `Standard`: fuente clásica FIGlet.
- `Slant`: fuente inclinada.
- `Small`: versión compacta y ligera.
- `Block`: estilo bloque sólido.
- `Larry3d`: aspecto tridimensional.
- `Lean`: diseño estrecho.

Para usar una de estas fuentes, llama a:

```rust
Banner::new("Tu texto").with_builtin_font(BuiltinFont::Larry3d);
```

## Contribuir

1. Haz un fork del repositorio.
2. Crea una rama (`git checkout -b feature/nueva-funcionalidad`).
3. Haz tus cambios y tests.
4. Envía un pull request.

¡Gracias por contribuir a Vornix Banner!