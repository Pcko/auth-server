use axum::extract::{ConnectInfo, FromRequestParts};
use domain::model::request_info::RequestInfo;
use http::{HeaderMap, header, request::Parts};
use std::{
    convert::Infallible,
    net::{IpAddr, SocketAddr},
};

#[derive(Debug, Clone)]
pub struct ExtractRequestInfo {
    pub url: String,
    pub user_agent: Option<String>,
    pub ip: Option<IpAddr>,
}

impl ExtractRequestInfo {
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

impl<S> FromRequestParts<S> for ExtractRequestInfo
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

impl From<ExtractRequestInfo> for RequestInfo {
    fn from(val: ExtractRequestInfo) -> Self {
        // from Option<IpAdrs> zu Option<String>
        let ip = val.ip.map(|addr| addr.to_string());

        Self {
            ip: ip,
            url: val.url,
            user_agent: val.user_agent,
        }
    }
}
