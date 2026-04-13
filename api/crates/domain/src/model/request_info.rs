#[derive(Debug, Clone)]
pub struct RequestInfo {
    pub url: String,
    pub user_agent: Option<String>,
    pub ip: Option<String>,
}
