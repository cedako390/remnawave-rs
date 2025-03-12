use serde::{Deserialize, Serialize};

/// Структура пользователя
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

/// Структура для создания нового пользователя
#[derive(Serialize)]
struct NewUser {
    name: String,
}