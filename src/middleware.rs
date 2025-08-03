use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use jsonwebtoken::{DecodingKey, Header, Validation, decode};
use log::{debug, error, info};
use serde::Deserialize;

use crate::AppState;

#[derive(Debug, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
    sub: String,
}

pub async fn jwt_auth(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match get_token(&headers) {
        Some(token) => {
            if is_valid_token(token, &state.secret) {
                let response = next.run(request).await;
                Ok(response)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    let header_key = "Authorization".to_string();
    let header_value = headers.get(header_key);

    match header_value {
        Some(raw_token) => match raw_token.to_str() {
            Ok(unstripped_token) => {
                let token = unstripped_token.strip_prefix("Bearer ");
                info!("Token extracted: {:?}", token);
                token
            }
            Err(_) => {
                error!("Error converting raw token");
                None
            }
        },

        None => {
            error!("Error getting header value");
            None
        }
    }
}

fn is_valid_token(token: &str, secret: &str) -> bool {
    info!("SECRET_CHUNGUS: {:?}", secret.to_string());
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();
    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(_) => {
            info!("Valid token");
            true
        }
        Err(_) => {
            error!("Invalid token");
            false
        }
    }
}
