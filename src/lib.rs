//! SigmaOS Aether - Native theming engine

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Color(pub String);

impl Color {
    pub fn new(hex: &str) -> Result<Self, String> {
        if hex.starts_with('#') && (hex.len() == 7 || hex.len() == 4) {
            Ok(Self(hex.to_string()))
        } else {
            Err(format!("Invalid hex color: {}", hex))
        }
    }

    pub fn hex(&self) -> &str {
        &self.0
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        let hex = self.0.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        (r, g, b)
    }
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub name: String,
    colors: HashMap<String, Color>,
}

impl Palette {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            colors: HashMap::new(),
        }
    }

    pub fn set(&mut self, role: &str, color: Color) {
        self.colors.insert(role.to_string(), color);
    }

    pub fn get(&self, role: &str) -> Option<&Color> {
        self.colors.get(role)
    }

    pub fn role_count(&self) -> usize {
        self.colors.len()
    }

    pub fn to_css_vars(&self) -> String {
        let mut lines: Vec<String> = self
            .colors
            .iter()
            .map(|(role, color)| format!("  --{}: {};", role, color.hex()))
            .collect();
        lines.sort();
        format!(":root {{\n{}\n}}", lines.join("\n"))
    }
}

pub struct AetherEngine {
    palettes: Vec<Palette>,
    pub active: Option<String>,
}

impl AetherEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            palettes: Vec::new(),
            active: None,
        };
        let mut mocha = Palette::new("catppuccin-mocha");
        mocha.set("bg-primary", Color::new("#1e1e2e").unwrap());
        mocha.set("accent", Color::new("#89b4fa").unwrap());
        engine.palettes.push(mocha);
        engine
    }

    pub fn register_palette(&mut self, palette: Palette) {
        self.palettes.push(palette);
    }

    pub fn apply(&mut self, name: &str) -> Result<String, String> {
        if self.palettes.iter().any(|p| p.name == name) {
            self.active = Some(name.to_string());
            Ok(format!("Applied theme: {}", name))
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }

    pub fn palette_count(&self) -> usize {
        self.palettes.len()
    }
}

impl Default for AetherEngine {
    fn default() -> Self {
        Self::new()
    }
}
