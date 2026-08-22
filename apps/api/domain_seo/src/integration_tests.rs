//! Live PostgreSQL coverage is opt-in because it requires Docker.

#[cfg(test)]
mod tests {
    use crate::{
        handlers::head_assets::{CreateHeadAssetRequest, HeadAssetHandler, UpdateHeadAssetRequest},
        migrations::Migrator,
    };
    use domain_interface::AuthenticatedActor;
    use sea_orm_migration::MigratorTrait;
    use std::sync::Arc;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    #[tokio::test]
    #[ignore = "requires Docker for PostgreSQL testcontainer"]
    async fn migration_and_public_lifecycle_cover_disable_enable_update_delete() {
        let postgres = Postgres::default()
            .start()
            .await
            .expect("postgres container");
        let port = postgres
            .get_host_port_ipv4(5432)
            .await
            .expect("postgres port");
        let conn = sea_orm::Database::connect(format!(
            "postgres://postgres:postgres@127.0.0.1:{port}/postgres"
        ))
        .await
        .expect("postgres connection");
        Migrator::up(&conn, None).await.expect("SEO migration up");
        let handler = HeadAssetHandler { db: Arc::new(conn) };
        let actor = AuthenticatedActor {
            user_id: "admin".into(),
            email: None,
            primary_role: "authenticated".into(),
            app_roles: vec!["my-headless-cms-administrator".into()],
        };
        let created = handler
            .create(
                CreateHeadAssetRequest {
                    label: "gtag".into(),
                    html: "<script>window.dataLayer=[];</script>".into(),
                    enabled: false,
                    sort_order: 1,
                },
                &actor,
            )
            .await
            .expect("create");
        assert!(handler.public_list().await.expect("public read").is_empty());
        let enabled = handler
            .update(
                created.id,
                UpdateHeadAssetRequest {
                    label: created.label.clone(),
                    html: created.html.clone(),
                    enabled: true,
                    sort_order: 1,
                    row_version: created.row_version,
                },
                &actor,
            )
            .await
            .expect("enable");
        assert_eq!(handler.public_list().await.expect("public read").len(), 1);
        assert!(handler
            .update(
                enabled.id,
                UpdateHeadAssetRequest {
                    label: "stale".into(),
                    html: enabled.html.clone(),
                    enabled: true,
                    sort_order: 1,
                    row_version: 1
                },
                &actor
            )
            .await
            .is_err());
        handler.delete(enabled.id, &actor).await.expect("delete");
        assert!(handler.public_list().await.expect("public read").is_empty());
        Migrator::down(handler.db.as_ref(), None)
            .await
            .expect("SEO migration down");
    }
}
