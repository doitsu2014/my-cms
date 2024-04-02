use crate::schema::posts;
use diesel::{deserialize::Queryable, prelude::Insertable, PgConnection, Selectable};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub created_by: String,
    pub last_modified_at: Option<chrono::NaiveDateTime>,
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
            created_at: None,
            created_by,
            last_modified_at: None,
            last_modified_by: created_by,
        }
    }

    pub fn save(&self, conn: &PgConnection) -> Result<Post, diesel::result::Error> {
        diesel::insert_into(crate::schema::posts::table)
            .values(self)
            .get_result(conn)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_post_save() {
        env_logger::init();
        let docker = Cli::default();
        let postgres = docker.run(create_postgres(&redis_password));
        postgres.get_host_port_ipv4(5432);

        let connection_string: String = format!("", postgres.get_host_port_ipv4(6379));

        let redis_client = create_redis_client(&connection_string);
        let mut con = redis_client.get_connection().unwrap();

        let _: () = con.set("my_key", 42).unwrap();
        // please explain _: ()
        let result: i32 = con.get("my_key").unwrap();
        assert_eq!(result, 42);
    }

    /// Create a Redis module with `6.2-alpine` tag and custom password
    fn create_postgres() -> RunnableImage<Postgres> {
        RunnableImage::from(Postgres::default())
    }

    // #[test]
    // fn test_create_redis_connection() {
    //     env_logger::init();
    //     let docker = Cli::default();
    //     let redis_password = "21345";
    //     let redis_cluster = docker.run(create_redis(&redis_password));
    //     let connection_string: String = format!(
    //         "redis://{}@127.0.0.1:{}/0",
    //         &redis_password,
    //         redis_cluster.get_host_port_ipv4(6379)
    //     );
    //
    //     info!("Connection: {}", connection_string);
    //
    //     let redis_client = create_redis_client(&connection_string);
    //     let mut con = redis_client.get_connection().unwrap();
    //
    //     let _: () = con.set("my_key", 42).unwrap();
    //     // please explain _: ()
    //     let result: i32 = con.get("my_key").unwrap();
    //     assert_eq!(result, 42);
    // }
    //
    // /// Create a Redis module with `6.2-alpine` tag and custom password
    // fn create_redis(password: &str) -> RunnableImage<Redis> {
    //     RunnableImage::from(Redis::default())
    //         .with_tag("6.2-alpine")
    //         .with_env_var(("REDIS_PASSWORD", password))
    //         .with_env_var(("REDIS_HOST_PASSWORD", password))
    // }
}
