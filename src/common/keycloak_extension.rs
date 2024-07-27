use axum_keycloak_auth::decode::{Email, KeycloakToken, Profile, ProfileAndEmail};

pub trait ExtractKeyCloakToken {
    fn extract_profile_email(&self) -> ProfileAndEmail;
    fn extract_profile(&self) -> Profile;
    fn extract_email(&self) -> Email;
}

impl ExtractKeyCloakToken for KeycloakToken<String> {
    fn extract_profile_email(&self) -> ProfileAndEmail {
        self.extra.clone()
    }

    fn extract_profile(&self) -> Profile {
        self.extra.profile.clone()
    }

    fn extract_email(&self) -> Email {
        self.extra.email.clone()
    }
}
