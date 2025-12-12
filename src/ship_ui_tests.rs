use crate::data_types::{HullClass, Language, LocalizedText};
use crate::ship_ui::HullSelection;

fn localized(text: &str) -> LocalizedText {
    LocalizedText {
        en: text.to_string(),
        ru: text.to_string(),
    }
}

fn hull(id: &str, max_items: i32) -> HullClass {
    HullClass {
        id: id.to_string(),
        name: localized(id),
        description: localized("desc"),
        size_index: 1,
        max_items,
    }
}

#[test]
fn selects_first_by_default() {
    let selection = HullSelection::from_hulls(vec![hull("a", 1), hull("b", 2), hull("c", 3)]);
    assert_eq!(selection.selected_id(), Some("a"));
}

#[test]
fn cycles_selection_forward_and_backward() {
    let mut selection = HullSelection::from_hulls(vec![hull("a", 1), hull("b", 2), hull("c", 3)]);

    selection.next();
    assert_eq!(selection.selected_id(), Some("b"));
    selection.next();
    assert_eq!(selection.selected_id(), Some("c"));
    selection.next();
    assert_eq!(selection.selected_id(), Some("a"));

    selection.prev();
    assert_eq!(selection.selected_id(), Some("c"));
}

#[test]
fn renders_hull_list() {
    let selection = HullSelection::from_hulls(vec![hull("a", 1), hull("b", 2), hull("c", 3)]);
    let rendered = selection.render(Language::En);
    assert!(rendered.contains("Hull selection:"));
    assert!(rendered.contains("> a"));
    assert!(rendered.contains("max items"));
}
