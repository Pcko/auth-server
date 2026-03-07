#[derive(Debug)]
pub enum  SessionRepositoryError {
    NotFound,
    Conflict,
    Unexpected(String)
}


#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    
}
