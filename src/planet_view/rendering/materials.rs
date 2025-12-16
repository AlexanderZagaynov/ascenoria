use bevy::prelude::*;

/// Create a material for the planet.
pub fn create_planet_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    surface_type: &str,
) -> Handle<StandardMaterial> {
    let base_color = get_planet_base_color(surface_type);

    materials.add(StandardMaterial {
        base_color,
        // Use vertex colors
        base_color_texture: None,
        perceptual_roughness: 0.7,
        metallic: 0.0,
        reflectance: 0.3,
        ..default()
    })
}

/// Get the base color for a planet type.
pub fn get_planet_base_color(surface_type: &str) -> Color {
    match surface_type {
        "husk" => Color::srgb(0.2, 0.15, 0.15),
        "primordial" => Color::srgb(0.5, 0.45, 0.4),
        "congenial" => Color::srgb(0.6, 0.65, 0.6),
        "eden" => Color::srgb(0.3, 0.6, 0.35),
        "mineral" => Color::srgb(0.6, 0.4, 0.35),
        "supermineral" => Color::srgb(0.7, 0.35, 0.3),
        "chapel" | "cathedral" => Color::srgb(0.4, 0.5, 0.7),
        "special" => Color::srgb(0.6, 0.5, 0.65),
        "tycoon" => Color::srgb(0.75, 0.65, 0.4),
        "cornucopia" => Color::srgb(0.8, 0.7, 0.5),
        _ => Color::srgb(0.6, 0.6, 0.55),
    }
}

/// Get a representative color for planet thumbnails based on surface type.
pub fn get_planet_thumbnail_color(surface_type: &str) -> Color {
    use crate::planet_view::types::colors;
    match surface_type {
        "husk" => colors::TILE_BLACK,
        "primordial" => Color::srgb(0.4, 0.35, 0.3),
        "congenial" => colors::TILE_WHITE,
        "eden" => colors::TILE_GREEN,
        "mineral" | "supermineral" => colors::TILE_RED,
        "chapel" | "cathedral" => colors::TILE_BLUE,
        "special" => Color::srgb(0.6, 0.5, 0.6),
        "tycoon" => Color::srgb(0.7, 0.6, 0.3),
        "cornucopia" => Color::srgb(0.8, 0.7, 0.5),
        _ => colors::TILE_WHITE,
    }
}
