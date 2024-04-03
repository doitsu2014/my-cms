use diesel::{
    deserialize::Queryable, prelude::Insertable, Connection, PgConnection, QueryResult,
    RunQueryDsl, Selectable,
};

use crate::schema::posts;
use std::time::SystemTime;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
    pub created_at: SystemTime,
    pub created_by: String,
    pub last_modified_at: SystemTime,
    pub last_modified_by: String,
}

impl Post {
    pub fn new(title: String, slug: String, content: String, created_by: String) -> Self {
        Post {
            id: 0,
            title,
            slug,
            content,
            published: false,
            created_at: SystemTime::now(),
            created_by: created_by.to_owned(),
            last_modified_at: SystemTime::now(),
            last_modified_by: created_by,
        }
    }

    pub fn establish_connection(connection_string: &str) -> PgConnection {
        PgConnection::establish(connection_string)
            .expect(&format!("Error connecting to {}", connection_string))
    }

    pub fn save(&self, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::insert_into(crate::schema::posts::table)
            .values(self)
            .execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use crate::domains::post::Post;
    use testcontainers::{clients::Cli, RunnableImage};
    use testcontainers_modules::postgres::Postgres;

    #[test]
    fn test_post_save() {
        let docker = Cli::default();
        let postgres = docker.run(create_postgres());
        let connection_string: String = format!(
            "postgres://postgres@localhost:{}/postgres",
            postgres.get_host_port_ipv4(5432)
        );
        let mut conn = Post::establish_connection(&connection_string);
        let new_post = Post::new("Title 1".to_string(), "title-1".to_string(), "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.".to_string(), "Duc Tran".to_string());
        new_post.save(&mut conn).unwrap();
    }

    /// Create a Redis module with `6.2-alpine` tag and custom password
    fn create_postgres() -> RunnableImage<Postgres> {
        RunnableImage::from(Postgres::default())
    }
}
