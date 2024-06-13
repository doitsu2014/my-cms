use oauth2::{reqwest::{self, async_http_client}, HttpRequest};
use jsonwebtoken::jwk::{Jwk, JwkSet};

async fn fetch_jwks() -> JwkSet {
    let json_string = r#"
    {
        "keys": [
            {
                "kty": "RSA",
                "use": "sig",
                "kid": "54FE9DD58C43B3D4EC2BAC15C96B5DE2",
                "e": "AQAB",
                "n": "jSz6hO4NYHpCQKhJUsq4o87FRZ7fFs1B8gG_1H6X_fXwB3nXb5xxDtTZkUULwUKGZockEoLFZdDZm7cWVTbNAypML8_mvKEHaVQKeZHCAvEmU6aCcCyaDPJZkgkCT98jE213hqnbTMqO5rQwpHhsC2InUdGoeF1fiK-BK-11E6ooCSJGqrVdF59l6d9kI-GSXPQ-exA0lZTAAQkwmtevYsWZ0Nce8NssMRXst5jgwp1IsyPKLL01mDpGfhAOWvBMyHWaucSSpGVl1OOx6GERVDqhJq2zjdOPsUPX8jaercY2fLD8ADJpTgx1bZE8qROg9qvPJZNgiVAIfzNKfD-Faw",
                "alg": "RS256"
            }
        ]
    }
    "#;
    let result = serde_json::from_str::<JwkSet>(json_string).unwrap();
    result
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{DecodingKey};
    use serde::{Deserialize, Serialize};

    use crate::{authorization_handler::fetch_jwks, presentation_models::api_response::*};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        aud: String,         // Optional. Audience
        exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
        iat: usize,          // Optional. Issued at (as UTC timestamp)
        iss: String,         // Optional. Issuer
        nbf: usize,          // Optional. Not Before (as UTC timestamp)
    }

    #[tokio::test]
    async fn test_case_one() {
        let fetch_jwks_result = fetch_jwks().await;
        let first_jwk = fetch_jwks_result.keys.first().unwrap();
        let test_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjU0RkU5REQ1OEM0M0IzRDRFQzJCQUMxNUM5NkI1REUyIiwidHlwIjoiYXQrand0In0.eyJpc3MiOiJodHRwczovL2lkcy1zdHMuZG9pdHN1LnRlY2giLCJuYmYiOjE3MTgyNTc4MDcsImlhdCI6MTcxODI1NzgwNywiZXhwIjoxNzE4MjYxNDA3LCJhdWQiOiJkdGVjaC5zdXBlcmFkbWluX2FwaSIsInNjb3BlIjpbImR0ZWNoLnN1cGVyYWRtaW5fYXBpIl0sImNsaWVudF9pZCI6ImR0ZWNoLnN1cGVyYWRtaW4iLCJqdGkiOiIwMkUwQUVFREREMDY1MjIzNUQxNUZDNTI0RDhFNjMyRCJ9.U0xDm2sIKmKK4Df9M4SzbaBitJU0chPvXHOSlNIHJsld1LXdIurvF6e4zckhmAdwHTGLoffxGRTkujoXnZAvGZJOFa556rbGdA5tbPrGofrtCPyfgIvFMqZOp_GGCr9XrF1lJfmuf7S3xo7fhE80d7i9xCr6dNhTCy_P4qqdnvToOZx9l5ExV6pUgVDD0K9D9x2JmxHDqTRW_yfphMAVj29TPv9o1u2NZysKhd3Bz7yRazGQEoBnhmCyBLaLfy1GWv6g9fH7F3N3xtvKFa8zz2QM26RwtwirZ-mbKQvH2aAIJSQWCQ8xXcxjb2P_fJuzCPDqODXZeLdpQwsGd4aKoQ";

        let decoding_key = DecodingKey::from_jwk(first_jwk).unwrap();
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&["dtech.superadmin_api"]);

        let decoded = jsonwebtoken::decode::<Claims>(
            &test_token,
            &decoding_key,
            &validation,
        );

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