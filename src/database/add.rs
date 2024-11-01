use std::fs;

use super::*;

pub fn add_files_to_database(original_path: &str, copied_path: &str) {
    let database_path = get_database_path();

    if std::path::Path::new(&database_path).exists() {
        let mut table_struct: DataTable = get_table_struct();

        table_struct
            .table
            .entry(original_path.to_string())
            .or_insert(copied_path.to_string());

        let json_table = serde_json::to_string_pretty(&table_struct).unwrap();
        fs::write(&database_path, json_table).unwrap();
    } else {
        fs::File::create(&database_path).unwrap();

        let mut table_struct = DataTable::new();
        table_struct
            .table
            .entry(original_path.to_string())
            .or_insert(copied_path.to_string());
        let json_table = serde_json::to_string_pretty(&table_struct).unwrap();
        fs::write(&database_path, json_table).unwrap();
    }
}
