use toml_edit::visit_mut::{visit_table_like_kv_mut, VisitMut};
use toml_edit::{Item, KeyMut, Value};

pub struct HackyFormatter;

impl VisitMut for HackyFormatter {
    fn visit_table_like_kv_mut(&mut self, mut key: KeyMut<'_>, node: &mut Item) {
        // Convert inline tables to tables.
        // By default, toml_edit serializes all tables as inline (and arrays of tables as... an array of inline tables.)
        if let Item::Value(Value::InlineTable(table)) = node {
            *node = Item::Table(table.clone().into_table())
        }

        visit_table_like_kv_mut(self, key, node);
    }
}
