use reqwest::header::HeaderMap;

const ENDPOINT: &str = "prisma.cubedhost.com/api";

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

pub struct CubedHostClient<'a> {
    pub server_id: &'a str,
    pub api_key: &'a str,
    pub api_user: &'a str,
}

impl CubedHostClient<'_> {
    pub async fn get_server_config(&self) -> () {
        let body = reqwest::Client::new()
            .get(format!("{}/server/{}", ENDPOINT, self.server_id).as_str())
            .headers(self.construct_headers())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{:?}", body)
    }

    fn construct_headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(Headers::ApiKey.as_str(), self.api_key.parse().unwrap());
        h.insert(Headers::ApiUser.as_str(), self.api_user.parse().unwrap());
        h
    }
}
