use crate::common::supabase_auth::SupabaseToken;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension,
};
use sea_orm::sqlx::types::Uuid;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    presentation_models::api_response::ErrorCode, ApiResponseError, ApiResponseWith, AppState,
    AxumResponse,
};

use application_core::entities::translation_jobs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatusResponse {
    pub job_id: String,
    pub post_id: String,
    pub target_language: String,
    pub status: String,
    pub progress: i32,
    pub error_message: Option<String>,
    pub ai_model: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveJobsResponse {
    pub jobs: Vec<JobStatusResponse>,
}

/// Get status of a specific translation job
///
/// GET /posts/{post_id}/translate/jobs/{job_id}
#[instrument]
pub async fn api_get_job_status(
    state: State<AppState>,
    Extension(_token): Extension<SupabaseToken>,
    Path((post_id, job_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let job_result = translation_jobs::Entity::find_by_id(job_id)
        .filter(translation_jobs::Column::PostId.eq(post_id))
        .one(state.conn.as_ref())
        .await;

    let job = match job_result {
        Ok(Some(job)) => job,
        Ok(None) => {
            return ApiResponseError::new()
                .with_error_code(ErrorCode::NotFound)
                .add_error("Job not found".to_string())
                .to_axum_response();
        }
        Err(e) => {
            tracing::error!("Database error when fetching job: {}", e);
            return ApiResponseError::new()
                .with_error_code(ErrorCode::ConnectionError)
                .add_error(format!("Failed to fetch job: {}", e))
                .to_axum_response();
        }
    };

    let response = JobStatusResponse {
        job_id: job.id.to_string(),
        post_id: job.post_id.to_string(),
        target_language: job.target_language,
        status: job.status,
        progress: job.progress,
        error_message: job.error_message,
        ai_model: job.ai_model,
        created_at: job.created_at.to_string(),
        updated_at: job.updated_at.to_string(),
    };

    ApiResponseWith::new(response).to_axum_response()
}

/// Get all active translation jobs for a post (pending or processing)
///
/// GET /posts/{post_id}/translate/jobs
#[instrument]
pub async fn api_get_active_jobs(
    state: State<AppState>,
    Extension(_token): Extension<SupabaseToken>,
    Path(post_id): Path<Uuid>,
) -> impl IntoResponse {
    let jobs_result = translation_jobs::Entity::find()
        .filter(translation_jobs::Column::PostId.eq(post_id))
        .filter(
            translation_jobs::Column::Status
                .eq("pending")
                .or(translation_jobs::Column::Status.eq("processing")),
        )
        .all(state.conn.as_ref())
        .await;

    let jobs = match jobs_result {
        Ok(jobs) => jobs,
        Err(e) => {
            tracing::error!("Database error when fetching jobs: {}", e);
            return ApiResponseError::new()
                .with_error_code(ErrorCode::ConnectionError)
                .add_error(format!("Failed to fetch jobs: {}", e))
                .to_axum_response();
        }
    };

    let job_responses: Vec<JobStatusResponse> = jobs
        .into_iter()
        .map(|job| JobStatusResponse {
            job_id: job.id.to_string(),
            post_id: job.post_id.to_string(),
            target_language: job.target_language,
            status: job.status,
            progress: job.progress,
            error_message: job.error_message,
            ai_model: job.ai_model,
            created_at: job.created_at.to_string(),
            updated_at: job.updated_at.to_string(),
        })
        .collect();

    ApiResponseWith::new(ActiveJobsResponse {
        jobs: job_responses,
    })
    .to_axum_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::DatabaseConnection;
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    async fn create_test_post(db: Arc<DatabaseConnection>) -> Uuid {
        use application_core::entities::{categories, posts, sea_orm_active_enums::CategoryType};
        use chrono::Utc;
        use sea_orm::{ActiveModelTrait, Set};

        // Create a category first
        let category = categories::ActiveModel {
            id: Set(Uuid::new_v4()),
            display_name: Set("Test Category".to_string()),
            slug: Set("test-category".to_string()),
            category_type: Set(CategoryType::Blog),
            created_by: Set("test-user".to_string()),
            created_at: Set(Utc::now().into()),
            last_modified_by: Set(None),
            last_modified_at: Set(None),
            parent_id: Set(None),
            row_version: Set(0),
        };
        let category_id = category.insert(db.as_ref()).await.unwrap().id;

        // Create a post
        let post = posts::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set("Test Post".to_string()),
            slug: Set("test-post".to_string()),
            preview_content: Set(Some("Preview content".to_string())),
            content: Set("Test content".to_string()),
            thumbnail_paths: Set(vec![]),
            published: Set(false),
            created_by: Set("test-user".to_string()),
            created_at: Set(Utc::now().into()),
            last_modified_by: Set(None),
            last_modified_at: Set(None),
            category_id: Set(category_id),
            row_version: Set(0),
        };
        post.insert(db.as_ref()).await.unwrap().id
    }

    #[async_std::test]
    async fn test_get_job_status_success() {
        let test_space = setup_test_space().await;
        let database: DatabaseConnection = test_space.postgres.get_database_connection().await;
        let arc_conn = Arc::new(database);

        // Create a test post first to satisfy foreign key constraint
        let post_id = create_test_post(arc_conn.clone()).await;

        // Create a test job
        use application_core::entities::translation_jobs;
        let job_id = Uuid::new_v4();

        let job = translation_jobs::ActiveModel {
            id: sea_orm::Set(job_id),
            post_id: sea_orm::Set(post_id),
            target_language: sea_orm::Set("vi".to_string()),
            status: sea_orm::Set("processing".to_string()),
            progress: sea_orm::Set(50),
            error_message: sea_orm::Set(None),
            ai_model: sea_orm::Set("gpt-4o-mini".to_string()),
            created_at: sea_orm::Set(chrono::Utc::now().into()),
            updated_at: sea_orm::Set(chrono::Utc::now().into()),
        };

        translation_jobs::Entity::insert(job)
            .exec(arc_conn.as_ref())
            .await
            .unwrap();

        // Query the job
        let result = translation_jobs::Entity::find_by_id(job_id)
            .filter(translation_jobs::Column::PostId.eq(post_id))
            .one(arc_conn.as_ref())
            .await
            .unwrap();

        assert!(result.is_some());
        let job = result.unwrap();
        assert_eq!(job.status, "processing");
        assert_eq!(job.progress, 50);
        assert_eq!(job.ai_model, "gpt-4o-mini");
    }

    #[async_std::test]
    async fn test_get_job_status_not_found() {
        let test_space = setup_test_space().await;
        let database: DatabaseConnection = test_space.postgres.get_database_connection().await;
        let arc_conn = Arc::new(database);

        let non_existent_job_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();

        // Query non-existent job
        let result = translation_jobs::Entity::find_by_id(non_existent_job_id)
            .filter(translation_jobs::Column::PostId.eq(post_id))
            .one(arc_conn.as_ref())
            .await
            .unwrap();

        assert!(result.is_none());
    }

    #[async_std::test]
    async fn test_get_active_jobs() {
        let test_space = setup_test_space().await;
        let database: DatabaseConnection = test_space.postgres.get_database_connection().await;
        let arc_conn = Arc::new(database);

        // Create a test post first to satisfy foreign key constraint
        let post_id = create_test_post(arc_conn.clone()).await;

        // Create jobs with different statuses
        use application_core::entities::translation_jobs;

        let jobs_data = vec![
            ("pending", 0, "vi"),
            ("processing", 50, "ja"),
            ("completed", 100, "ko"),
            ("failed", 25, "zh"),
        ];

        for (status, progress, lang) in jobs_data {
            let job = translation_jobs::ActiveModel {
                id: sea_orm::Set(Uuid::new_v4()),
                post_id: sea_orm::Set(post_id),
                target_language: sea_orm::Set(lang.to_string()),
                status: sea_orm::Set(status.to_string()),
                progress: sea_orm::Set(progress),
                error_message: sea_orm::Set(None),
                ai_model: sea_orm::Set("gpt-4o-mini".to_string()),
                created_at: sea_orm::Set(chrono::Utc::now().into()),
                updated_at: sea_orm::Set(chrono::Utc::now().into()),
            };

            translation_jobs::Entity::insert(job)
                .exec(arc_conn.as_ref())
                .await
                .unwrap();
        }

        // Query active jobs
        let active_jobs = translation_jobs::Entity::find()
            .filter(translation_jobs::Column::PostId.eq(post_id))
            .filter(
                translation_jobs::Column::Status
                    .eq("pending")
                    .or(translation_jobs::Column::Status.eq("processing")),
            )
            .all(arc_conn.as_ref())
            .await
            .unwrap();

        // Should have 2 active jobs
        assert_eq!(active_jobs.len(), 2);

        let statuses: Vec<String> = active_jobs.iter().map(|j| j.status.clone()).collect();
        assert!(statuses.contains(&"pending".to_string()));
        assert!(statuses.contains(&"processing".to_string()));
    }

    #[async_std::test]
    async fn test_job_progress_updates() {
        let test_space = setup_test_space().await;
        let database: DatabaseConnection = test_space.postgres.get_database_connection().await;
        let arc_conn = Arc::new(database);

        // Create a test post first to satisfy foreign key constraint
        let post_id = create_test_post(arc_conn.clone()).await;

        use application_core::entities::translation_jobs;
        let job_id = Uuid::new_v4();

        // Create job at 0%
        let job = translation_jobs::ActiveModel {
            id: sea_orm::Set(job_id),
            post_id: sea_orm::Set(post_id),
            target_language: sea_orm::Set("vi".to_string()),
            status: sea_orm::Set("pending".to_string()),
            progress: sea_orm::Set(0),
            error_message: sea_orm::Set(None),
            ai_model: sea_orm::Set("gpt-4o-mini".to_string()),
            created_at: sea_orm::Set(chrono::Utc::now().into()),
            updated_at: sea_orm::Set(chrono::Utc::now().into()),
        };

        translation_jobs::Entity::insert(job)
            .exec(arc_conn.as_ref())
            .await
            .unwrap();

        // Simulate progress updates
        let progress_steps = vec![10, 30, 50, 75, 100];
        let status_steps = vec![
            "processing",
            "processing",
            "processing",
            "processing",
            "completed",
        ];

        for (progress, status) in progress_steps.iter().zip(status_steps.iter()) {
            use sea_orm::ActiveModelTrait;

            let job = translation_jobs::Entity::find_by_id(job_id)
                .one(arc_conn.as_ref())
                .await
                .unwrap()
                .unwrap();

            let mut active_job: translation_jobs::ActiveModel = job.into();
            active_job.progress = sea_orm::Set(*progress);
            active_job.status = sea_orm::Set(status.to_string());
            active_job.updated_at = sea_orm::Set(chrono::Utc::now().into());

            active_job.update(arc_conn.as_ref()).await.unwrap();

            // Verify update
            let updated = translation_jobs::Entity::find_by_id(job_id)
                .one(arc_conn.as_ref())
                .await
                .unwrap()
                .unwrap();

            assert_eq!(updated.progress, *progress);
            assert_eq!(updated.status, *status);
        }
    }

    #[async_std::test]
    async fn test_multiple_language_jobs() {
        let test_space = setup_test_space().await;
        let database: DatabaseConnection = test_space.postgres.get_database_connection().await;
        let arc_conn = Arc::new(database);

        // Create a test post first to satisfy foreign key constraint
        let post_id = create_test_post(arc_conn.clone()).await;

        // Create jobs for different languages
        use application_core::entities::translation_jobs;
        let languages = vec!["vi", "ja", "ko", "zh", "fr"];

        for lang in languages.iter() {
            let job = translation_jobs::ActiveModel {
                id: sea_orm::Set(Uuid::new_v4()),
                post_id: sea_orm::Set(post_id),
                target_language: sea_orm::Set(lang.to_string()),
                status: sea_orm::Set("processing".to_string()),
                progress: sea_orm::Set(25),
                error_message: sea_orm::Set(None),
                ai_model: sea_orm::Set("gpt-4o-mini".to_string()),
                created_at: sea_orm::Set(chrono::Utc::now().into()),
                updated_at: sea_orm::Set(chrono::Utc::now().into()),
            };

            translation_jobs::Entity::insert(job)
                .exec(arc_conn.as_ref())
                .await
                .unwrap();
        }

        // Query all jobs for this post
        let all_jobs = translation_jobs::Entity::find()
            .filter(translation_jobs::Column::PostId.eq(post_id))
            .all(arc_conn.as_ref())
            .await
            .unwrap();

        assert_eq!(all_jobs.len(), languages.len());

        // Verify each language is present
        let job_languages: Vec<String> =
            all_jobs.iter().map(|j| j.target_language.clone()).collect();
        for lang in languages {
            assert!(job_languages.contains(&lang.to_string()));
        }
    }
}
