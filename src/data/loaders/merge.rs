use std::collections::BTreeMap;

use crate::data::entities::TechEdge;

pub(crate) fn merge_by_id<T, F>(base: &mut Vec<T>, mods: Vec<T>, id_fn: F)
where
    F: Fn(&T) -> &str,
{
    for item in mods {
        let id = id_fn(&item);
        if let Some(pos) = base.iter().position(|b| id_fn(b) == id) {
            base[pos] = item;
        } else {
            base.push(item);
        }
    }
}

pub(crate) fn merge_tech_edges(base: &mut Vec<TechEdge>, mods: Vec<TechEdge>) {
    if mods.is_empty() {
        return;
    }

    let mut merged: BTreeMap<(String, String), TechEdge> = BTreeMap::new();
    for edge in base.drain(..) {
        merged.insert((edge.from.clone(), edge.to.clone()), edge);
    }
    for edge in mods {
        merged.insert((edge.from.clone(), edge.to.clone()), edge);
    }
    *base = merged.into_values().collect();
}
