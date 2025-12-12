use crate::data::entities::HullClass;
use crate::data::localization::{HasDescription, Language, LocalizedText, NamedEntity};

#[test]
fn localized_entity_helpers_resolve_language() {
    let hull = HullClass {
        id: "frigate".to_string(),
        name: LocalizedText {
            en: "Frigate".to_string(),
            ru: "Фрегат".to_string(),
        },
        description: LocalizedText {
            en: "Light hull".to_string(),
            ru: "Легкий корпус".to_string(),
        },
        size_index: 1,
        max_items: 4,
    };

    assert_eq!(hull.name(Language::En), "Frigate");
    assert_eq!(hull.name(Language::Ru), "Фрегат");
    assert_eq!(
        HasDescription::description(&hull, Language::En),
        "Light hull"
    );
    assert_eq!(
        HasDescription::description(&hull, Language::Ru),
        "Легкий корпус"
    );
}
