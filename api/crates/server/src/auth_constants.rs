pub mod auth_constants {
    use tower_cookies::cookie::SameSite;

    pub const ACCESS_COOKIE_NAME: &str = "accessToken";
    pub const REFRESH_COOKIE_NAME: &str = "refreshToken";

    //TODO implement the config when creating or deleting cookies
    pub struct CookieConfig {
        pub domain: Option<String>,
        pub secure: bool,
        pub same_site: SameSite,
        pub path: String,
    }
}