#[cfg(test)]
mod tests {

    use chrono::Utc;
    use entity::{prelude::*, *};
    use migration::Migrator;
    use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, Database, EntityTrait};
    use sea_orm_migration::prelude::*;
    use testcontainers::{clients::Cli, RunnableImage};
    use testcontainers_modules::postgres::Postgres;

    #[async_std::test]
    async fn test_post_save() {
        let docker = Cli::default();
        let postgres = docker.run(create_postgres());
        let connection_string: String = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres.get_host_port_ipv4(5432)
        );

        let titel = post::ActiveModel {
            id: ActiveValue::Set(1),
            title: ActiveValue::Set("Tiele 1".to_string()),
            slug: ActiveValue::Set("title-1".to_string()),
            content: ActiveValue::Set("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.".to_string()),
            published: ActiveValue::Set(true),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            created_by: ActiveValue::Set("Duc Tran".to_string()),
            last_modified_at: ActiveValue::Set(Utc::now().naive_utc()),
            last_modified_by: ActiveValue::Set("Duc Tran".to_string()),
        };

        let opt = ConnectOptions::new(connection_string);
        let db = Database::connect(opt).await.unwrap();
        Migrator::refresh(&db).await.unwrap();

        titel.insert(&db).await.unwrap();

        let inserted_titel = Post::find().one(&db).await.unwrap().unwrap();
        assert_eq!(inserted_titel.id, 1);
    }

    /// Create a Redis module with `6.2-alpine` tag and custom password
    fn create_postgres() -> RunnableImage<Postgres> {
        RunnableImage::from(Postgres::default())
    }
}
