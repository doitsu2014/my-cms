use application_core::{
    entities::tags::{self, ActiveModel, Column, Model},
    StringExtension, Tags,
};
use migration::Expr;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use tracing::instrument;

use super::read_response::GetAndClassifyTagCommandResponse;

#[instrument]
pub async fn handle_get_tags_by_names(
    conn: DatabaseConnection,
    names: Vec<String>,
) -> Result<Vec<tags::Model>, DbErr> {
    // Get all tags with names
    let tags = Tags::find()
        .filter(Expr::col(Column::Name).is_in(names))
        .all(&conn)
        .await?;
    Ok(tags)
}

#[instrument]
pub async fn handle_get_and_classify_tags_by_names(
    conn: DatabaseConnection,
    names: Vec<String>,
) -> Result<GetAndClassifyTagCommandResponse, DbErr> {
    if names.is_empty() {
        return Ok(GetAndClassifyTagCommandResponse {
            new_tags: vec![],
            existing_tags: vec![],
        });
    }

    // Get all tags with names
    let tags = handle_get_tags_by_names(conn, names.clone()).await?;
    let mut new_tags: Vec<ActiveModel> = vec![];
    for name in names {
        let slug = name.to_slug();
        if tags.iter().all(|tag| tag.name != name) {
            let tag = ActiveModel {
                name: Set(name.clone()),
                slug: Set(slug),
                ..Default::default()
            };
            new_tags.push(tag);
        }
    }
    Ok(GetAndClassifyTagCommandResponse {
        new_tags,
        existing_tags: tags,
    })
}

#[instrument]
pub async fn handle_get_tags_by_slugs(
    conn: DatabaseConnection,
    slugs: Vec<String>,
) -> Result<Vec<tags::Model>, DbErr> {
    // Get all tags with names
    let tags = Tags::find()
        .filter(Expr::col(Column::Slug).is_in(slugs))
        .all(&conn)
        .await?;
    Ok(tags)
}
