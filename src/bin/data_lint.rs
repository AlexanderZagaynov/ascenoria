use std::env;

#[path = "../data.rs"]
mod data;

use data::{LocalizedEntity, LocalizedText, load_game_data};

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "assets/data".to_string());

    let (game_data, _) = match load_game_data(&path) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Validation failed for {}: {err}", path);
            std::process::exit(1);
        }
    };

    let mut warnings = Vec::new();

    lint_localized(
        "species",
        game_data.species(),
        |s| s.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "planet_size",
        game_data.planet_sizes(),
        |p| p.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "planet_surface_type",
        game_data.planet_surface_types(),
        |p| p.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "surface_item",
        game_data.surface_items(),
        |i| i.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "orbital_item",
        game_data.orbital_items(),
        |i| i.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "planetary_project",
        game_data.planetary_projects(),
        |p| p.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "hull_class",
        game_data.hull_classes(),
        |h| h.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "engine",
        game_data.engines(),
        |e| e.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "weapon",
        game_data.weapons(),
        |w| w.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "shield",
        game_data.shields(),
        |s| s.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "scanner",
        game_data.scanners(),
        |s| s.id.as_str(),
        &mut warnings,
    );
    lint_localized(
        "special_module",
        game_data.special_modules(),
        |s| s.id.as_str(),
        &mut warnings,
    );
    lint_localized("tech", game_data.techs(), |t| t.id.as_str(), &mut warnings);
    lint_localized(
        "victory_condition",
        game_data.victory_conditions(),
        |v| v.id.as_str(),
        &mut warnings,
    );

    if warnings.is_empty() {
        println!("{}: validation and lint checks passed", path);
    } else {
        for warning in &warnings {
            eprintln!("warning: {}", warning);
        }
        println!("{}: completed with {} warnings", path, warnings.len());
    }
}

fn lint_localized<T, F>(kind: &str, items: &[T], id_fn: F, warnings: &mut Vec<String>)
where
    T: LocalizedEntity,
    F: Fn(&T) -> &str,
{
    for item in items {
        let id = id_fn(item);
        if !is_snake_case(id) {
            warnings.push(format!(
                "{kind} '{id}': id should use snake_case with a-z, 0-9, and underscores"
            ));
        }
        lint_localization(kind, id, item.name_text(), warnings);
        lint_localization(kind, id, item.description_text(), warnings);
    }
}

fn lint_localization(kind: &str, id: &str, text: &LocalizedText, warnings: &mut Vec<String>) {
    if text.en.trim().is_empty() {
        warnings.push(format!("{kind} '{id}': missing English localization entry"));
    }
    if text.ru.trim().is_empty() {
        warnings.push(format!("{kind} '{id}': missing Russian localization entry"));
    }
}

fn is_snake_case(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}
