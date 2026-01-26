use axum::{extract::{Path, State}, response::IntoResponse, Extension};
use axum_keycloak_auth::decode::KeycloakToken;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use sea_orm::sqlx::types::Uuid;
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
    Extension(_token): Extension<KeycloakToken<String>>,
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
    Extension(_token): Extension<KeycloakToken<String>>,
    Path(post_id): Path<Uuid>,
) -> impl IntoResponse {
    let jobs_result = translation_jobs::Entity::find()
        .filter(translation_jobs::Column::PostId.eq(post_id))
        .filter(
            translation_jobs::Column::Status.eq("pending")
                .or(translation_jobs::Column::Status.eq("processing"))
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
