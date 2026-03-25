use indexmap::IndexSet;

pub fn get_builtin_scalars() -> IndexSet<String> {
    return IndexSet::from_iter([
        "ID".into(),
        "Int".into(),
        "Float".into(),
        "Boolean".into(),
        "String".into(),
    ]);
}
