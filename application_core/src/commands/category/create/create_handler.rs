use super::create_request::CreateCategoryRequest;
use crate::{
    common::datetime_generator::generate_vietname_now,
    entities::{categories, category_tags, tags},
    Categories, Tags,
};
use crate::{
    keycloak_extension::ExtractKeyCloakToken,
    tag::read::read_handler::handle_get_and_classify_tags_by_names, ApiResponseError,
    ApiResponseWith, AppState, AxumResponse, ErrorCode,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use sea_orm::{
    prelude::Uuid, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set, TransactionError,
    TransactionTrait,
};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn handle_create_category_with_tags(
    conn: DatabaseConnection,
    body: CreateCategoryRequest,
    actor_email: Option<String>,
) -> Result<Uuid, TransactionError<DbErr>> {
    // Prepare Category
    let model: categories::Model = body.into_model();
    let model = categories::Model {
        created_by: actor_email.clone().unwrap_or("System".to_string()),
        created_at: generate_vietname_now(),
        ..model
    };
    let create_category = categories::ActiveModel {
        ..model.into_active_model()
    };
    // Prepare Tags
    let tags: Vec<String> = body.tags.unwrap_or_default();
    let classifed_tags = handle_get_and_classify_tags_by_names(conn.clone(), tags).await?;
    let existing_tag_ids = classifed_tags
        .existing_tags
        .iter()
        .map(|tag| tag.id)
        .collect::<Vec<Uuid>>();

    let result: Result<Uuid, TransactionError<DbErr>> = conn
        .transaction::<_, Uuid, DbErr>(|tx| {
            Box::pin(async move {
                // Insert Category
                let inserted_category = Categories::insert(create_category).exec(tx).await?;

                // Insert New Tags
                let mut new_tag_ids: Vec<Uuid> = vec![];
                if !classifed_tags.new_tags.is_empty() {
                    let new_tags = classifed_tags
                        .new_tags
                        .iter()
                        .map(|tag| {
                            let id = Uuid::new_v4();
                            new_tag_ids.push(id);
                            tags::ActiveModel {
                                id: Set(id),
                                name: tag.name.clone(),
                                slug: tag.slug.clone(),
                                created_by: Set(actor_email
                                    .clone()
                                    .unwrap_or("System".to_string())),
                                created_at: Set(generate_vietname_now()),
                                ..Default::default()
                            }
                        })
                        .collect::<Vec<tags::ActiveModel>>();
                    Tags::insert_many(new_tags).exec(tx).await?;
                }

                // Combine New Tag Ids and Existing Tag Ids
                let all_tags = existing_tag_ids
                    .into_iter()
                    .chain(new_tag_ids.into_iter())
                    .collect::<Vec<Uuid>>();

                // Insert Category Tags
                if !all_tags.is_empty() {
                    let category_tags = all_tags
                        .iter()
                        .map(|tag_id| {
                            category_tags::Model {
                                id: Uuid::new_v4(),
                                category_id: inserted_category.last_insert_id,
                                tag_id: tag_id.to_owned(),
                            }
                            .into_active_model()
                        })
                        .collect::<Vec<category_tags::ActiveModel>>();

                    category_tags::Entity::insert_many(category_tags)
                        .exec(tx)
                        .await?;
                }

                Ok(inserted_category.last_insert_id)
            })
        })
        .await;

    match result {
        Ok(inserted_id) => Ok(inserted_id),
        Err(e) => Err(e),
    }
}
