// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_sql::{Migration, MigrationKind};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE books (id INTEGER PRIMARY KEY, isbn TEXT, title TEXT, price NUMBER); CREATE TABLE orders (id INTEGER PRIMARY KEY, orderDate DATETIME, lines TEXT); CREATE TABLE invoices (id INTEGER PRIMARY KEY, invoiceDate DATETIME, lines TEXT); CREATE TABLE creditnotes (id INTEGER PRIMARY KEY, creditnoteDate DATETIME, lines TEXT);",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "update_books_table",
            sql: "ALTER TABLE books ADD author TEXT; ALTER TABLE books ADD coverImageUrl TEXT;",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "update_orders_table",
            sql: "ALTER TABLE orders ADD customer TEXT; ALTER TABLE invoices ADD customer TEXT; ALTER TABLE creditnotes ADD customer TEXT;",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 4,
            description: "add_settings_table",
            sql: "CREATE TABLE settings (id INTEGER PRIMARY KEY, name TEXT, value TEXT);",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 5,
            description: "update_tables_with_company",
            sql: "ALTER TABLE orders ADD company TEXT; ALTER TABLE invoices ADD company TEXT; ALTER TABLE creditnotes ADD company TEXT;",
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:grid.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
