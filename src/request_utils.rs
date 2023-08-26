use lambda_http::{Body, Request};
use serde::de::DeserializeOwned;

#[tracing::instrument]
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

#[tracing::instrument]
pub fn get_header(event: &Request, header: &str) -> Result<String, String> {
    let header = event
        .headers()
        .get(header)
        .ok_or_else(|| format!("Missing header: {}", header))?
        .to_str()
        .map_err(|e| e.to_string())?
        .to_string();
    Ok(header)
}
