use axum::extract::{ConnectInfo, FromRequestParts};
use http::{HeaderMap, header, request::Parts};
use std::{
    convert::Infallible,
    net::{IpAddr, SocketAddr},
};

#[derive(Debug, Clone)]
pub struct RequestInfoExtractr {
    pub url: String,
    pub user_agent: Option<String>,
    pub ip: Option<IpAddr>,
}

impl RequestInfoExtractr {
    const X_FORWARDED_FOR: &str = "x-forwarded-for";
    const X_REAL_IP: &str = "x-real-ip"; // for nginx

    fn get_ip_from_headers(headers: &HeaderMap) -> Option<IpAddr> {
        // try to extract ip through x-forwarded-for header
        for value in headers.get_all(Self::X_FORWARDED_FOR).iter() {
            let value = value.to_str().ok()?;
            for part in value.split(',') {
                if let Ok(ip) = part.trim().parse::<IpAddr>() {
                    return Some(ip);
                }
            }
        }

        // TODO: else get from nginx proxy header (when implemented)
        headers
            .get(Self::X_REAL_IP)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.trim().parse::<IpAddr>().ok())
    }
}

impl<S> FromRequestParts<S> for RequestInfoExtractr
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user_agent = parts
            .headers
            .get(header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);

        let ip = if let Some(ip) = Self::get_ip_from_headers(&parts.headers) {
            Some(ip)
        } else {
            ConnectInfo::<SocketAddr>::from_request_parts(parts, state)
                .await
                .ok()
                .map(|ConnectInfo(addr)| addr.ip())
        };

        Ok(Self {
            url: parts.uri.path().to_owned(),
            user_agent,
            ip,
        })
    }
}
