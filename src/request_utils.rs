use lambda_http::{Body, Request};
use serde::de::DeserializeOwned;

pub fn get_body<T>(event: Request) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let body = event.body();
    let body = match body {
        Body::Text(body) => body.as_str(),
        _ => return Err("Invalid body".into()),
    };
    let result: T = serde_json::from_str(body).map_err(|e| e.to_string())?;
    Ok(result)
}