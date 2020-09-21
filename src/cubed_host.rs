use reqwest::header::HeaderMap;
use reqwest::Response;

const ENDPOINT: &str = "https://prisma.cubedhost.com/api";

enum Headers {
    ApiKey,
    ApiUser,
}

impl Headers {
    fn as_str(&self) -> &'static str {
        match *self {
            Headers::ApiKey => "X-API-KEY",
            Headers::ApiUser => "X-API-USER",
        }
    }
}

pub struct CubedHostClient {
    pub server_id: String,
    pub api_key: String,
    pub api_user: String,
}

impl CubedHostClient {
    pub async fn get_server_config(&self) -> Result<reqwest::Response, reqwest::Error> {
        reqwest::Client::new()
            .get(format!("{}/server/{}", ENDPOINT, self.server_id).as_str())
            .headers(self.construct_headers())
            .send()
            .await
    }

    pub async fn restart_server(&self) -> Result<Response, reqwest::Error> {
        reqwest::Client::new()
            .post(format!("{}/server/{}/restart", ENDPOINT, self.server_id).as_str())
            .headers(self.construct_headers())
            .send()
            .await?
            .error_for_status()
    }

    fn construct_headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(Headers::ApiKey.as_str(), self.api_key.parse().unwrap());
        h.insert(Headers::ApiUser.as_str(), self.api_user.parse().unwrap());
        h
    }
}
