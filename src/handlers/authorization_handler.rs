use jsonwebtoken::jwk::JwkSet;
use reqwest::Url;
use std::env;

async fn fetch_jwks() -> Result<JwkSet, reqwest::Error> {
    let ids_url_str = env::var("IDS_URL").unwrap_or("https://ids-sts.doitsu.tech".to_string());

    let ids_url = Url::parse(&ids_url_str)
        .map(|e| e.join(".well-known/openid-configuration/jwks").unwrap())
        .unwrap();

    let body = reqwest::get(ids_url).await?.bytes().await.unwrap();

    let jwk_set: JwkSet = serde_json::from_slice(&body).unwrap();
    Ok(jwk_set)
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::DecodingKey;
    use serde::{Deserialize, Serialize};

    use crate::{authorization_handler::fetch_jwks, presentation_models::api_response::*};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        aud: String, // Optional. Audience
        exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
        iat: usize, // Optional. Issued at (as UTC timestamp)
        iss: String, // Optional. Issuer
        nbf: usize, // Optional. Not Before (as UTC timestamp)
    }

    #[tokio::test]
    async fn test_case_one() {
        let fetch_jwks_result = fetch_jwks().await.unwrap();
        let first_jwk = fetch_jwks_result.keys.first().unwrap();
        let test_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjU0RkU5REQ1OEM0M0IzRDRFQzJCQUMxNUM5NkI1REUyIiwidHlwIjoiYXQrand0In0.eyJpc3MiOiJodHRwczovL2lkcy1zdHMuZG9pdHN1LnRlY2giLCJuYmYiOjE3MTgyNTc4MDcsImlhdCI6MTcxODI1NzgwNywiZXhwIjoxNzE4MjYxNDA3LCJhdWQiOiJkdGVjaC5zdXBlcmFkbWluX2FwaSIsInNjb3BlIjpbImR0ZWNoLnN1cGVyYWRtaW5fYXBpIl0sImNsaWVudF9pZCI6ImR0ZWNoLnN1cGVyYWRtaW4iLCJqdGkiOiIwMkUwQUVFREREMDY1MjIzNUQxNUZDNTI0RDhFNjMyRCJ9.U0xDm2sIKmKK4Df9M4SzbaBitJU0chPvXHOSlNIHJsld1LXdIurvF6e4zckhmAdwHTGLoffxGRTkujoXnZAvGZJOFa556rbGdA5tbPrGofrtCPyfgIvFMqZOp_GGCr9XrF1lJfmuf7S3xo7fhE80d7i9xCr6dNhTCy_P4qqdnvToOZx9l5ExV6pUgVDD0K9D9x2JmxHDqTRW_yfphMAVj29TPv9o1u2NZysKhd3Bz7yRazGQEoBnhmCyBLaLfy1GWv6g9fH7F3N3xtvKFa8zz2QM26RwtwirZ-mbKQvH2aAIJSQWCQ8xXcxjb2P_fJuzCPDqODXZeLdpQwsGd4aKoQ";

        let decoding_key = DecodingKey::from_jwk(first_jwk).unwrap();
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&["dtech.superadmin_api"]);

        let decoded = jsonwebtoken::decode::<Claims>(&test_token, &decoding_key, &validation);

        match decoded {
            Ok(_) => {
                println!("Token is valid");
            }
            Err(e) => {
                println!("Token is invalid: {:?}", e);
            }
        }
    }
}
