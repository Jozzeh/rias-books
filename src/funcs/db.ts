import Database from "tauri-plugin-sql-api";

export async function getMainDb() {
  // sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
  const db = await Database.load("sqlite:riasbooks.db");
  return db;
}
