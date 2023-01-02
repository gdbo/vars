// use crate::errors::Error;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub default: bool,
    pub permissions: i32,
}

impl Role {
    // pub async fn init(pool: &MySqlPool) -> Result<(), Error> {
    //     let _row = sqlx::query!(
    //         "INSERT INTO role(name, `default`, permissions) VALUES (?, ?, ?), (?, ?, ?);",
    //         "User",
    //         1,
    //         Permission::role_user(),
    //         "Admin",
    //         2,
    //         Permission::role_user(),
    //     )
    //     .execute(pool)
    //     .await?
    //     .last_insert_id();

    //     Ok(())
    // }

    // pub fn has_permission(self, perm: i32) -> bool {
    //     (self.permissions & perm) == perm
    // }
}

#[derive(Clone, Copy)]
pub enum Permission {
    Follow,   // follow other user
    Comment,  // comment other user's article
    Write,    // write article
    Moderate, // manager other comment
    Admin,    // administrator
}

impl Permission {
    // fn role_user() -> i32 {
    //     let permissions = vec![Permission::Comment].iter().map(|&x| x as i32).sum();
    //     permissions
    // }

    // fn role_admin() -> i32 {
    //     vec![
    //         Permission::Comment,
    //         Permission::Write,
    //         Permission::Moderate,
    //         Permission::Admin,
    //     ]
    //     .iter()
    //     .map(|&x| x as i32)
    //     .sum()
    // }
}

impl From<i32> for Permission {
    fn from(value: i32) -> Self {
        match value {
            1 => Permission::Follow,
            2 => Permission::Comment,
            4 => Permission::Write,
            8 => Permission::Moderate,
            16 => Permission::Admin,
            _ => Permission::Follow,
        }
    }
}
