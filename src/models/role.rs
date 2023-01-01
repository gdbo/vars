use sqlx::FromRow;

#[derive(FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub default: bool,
    pub permissions: i32,
}
