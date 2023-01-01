use sqlx::FromRow;

#[derive(FromRow)]
pub struct Reply {
    pub id: i32,
    pub article_id: i32,
    pub tag_id: i32,
}
