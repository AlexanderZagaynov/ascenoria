use crate::data_types::errors::DataLoadError;
use std::collections::HashMap;

pub(crate) fn build_typed_index<T, F, I>(
    kind: &'static str,
    items: &[T],
    id_fn: F,
) -> Result<HashMap<I, usize>, DataLoadError>
where
    F: Fn(&T) -> I,
    I: Eq + std::hash::Hash + Clone + Into<String>,
{
    let mut index = HashMap::new();
    for (i, item) in items.iter().enumerate() {
        let id = id_fn(item);
        if index.insert(id.clone(), i).is_some() {
            return Err(DataLoadError::DuplicateId {
                kind,
                id: id.into(),
            });
        }
    }
    Ok(index)
}
