use sigmaos_linux_aether::{AetherEngine, Color, Palette};

#[test]
fn test_color_rgb_aaa() {
    // Arrange
    let c = Color::new("#1e1e2e").unwrap();
    // Act
    let (r, g, b) = c.to_rgb();
    // Assert
    assert_eq!(r, 0x1e);
    assert_eq!(g, 0x1e);
    assert_eq!(b, 0x2e);
}

#[test]
fn test_palette_css_aaa() {
    // Arrange
    let mut p = Palette::new("test");
    p.set("bg", Color::new("#000000").unwrap());
    // Act
    let css = p.to_css_vars();
    // Assert
    assert!(css.contains("--bg: #000000;"));
}

#[test]
fn test_aether_apply_aaa() {
    // Arrange
    let mut engine = AetherEngine::new();
    // Act
    let res = engine.apply("catppuccin-mocha");
    // Assert
    assert!(res.is_ok());
    assert_eq!(engine.active.as_deref(), Some("catppuccin-mocha"));
}
