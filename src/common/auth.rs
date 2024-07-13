use axum::Router;
use axum_keycloak_auth::{instance::KeycloakAuthInstance, layer::KeycloakAuthLayer};

pub enum Role {
    Administrator,
    Unknown(String),
}
