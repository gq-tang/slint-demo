use sqlx::prelude::FromRow;

pub mod ui {
    slint::include_modules!();
}

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub age: i32,
    pub gender: String,
    pub email: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

impl From<ui::User> for User {
    fn from(user: ui::User) -> Self {
        let email = if user.email.is_empty() {
            None
        } else {
            Some(user.email.to_string())
        };
        User {
            id: user.id,
            username: user.name.to_string(),
            age: user.age,
            gender: user.gender.to_string(),
            email: email,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<User> for ui::User {
    fn from(user: User) -> Self {
        ui::User {
            id: user.id,
            name: slint::SharedString::from(user.username),
            age: user.age,
            gender: slint::SharedString::from(user.gender),
            email: slint::SharedString::new(),
        }
    }
}
