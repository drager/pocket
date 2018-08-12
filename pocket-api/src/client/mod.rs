use failure::Error;
use futures::future::Future;
use httper;
use serde_json;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PocketClient {
    api_key: String,
    api_version: u32,
    http_client: httper::client::HttperClient,
    headers: httper::client::Headers,
    code: Option<Code>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PocketItem {
    title: String,
    url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    access_token: String,
    username: String,
}

impl User {
    pub fn new(access_token: &str, username: &str) -> Self {
        User {
            access_token: access_token.to_string(),
            username: username.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Step1 {
    pub code: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
enum DetailType {
    Complete,
    Simple,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
struct SignInParams {
    consumer_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RetriveParams {
    consumer_key: String,
    access_token: String,
    count: u32,
    //#[serde(rename = "detailType")]
    //detail_type: DetailType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Code {
    code: String,
}

const BASE_URL: &str = "https://getpocket.com";

impl PocketClient {
    pub fn new(api_key: &str, api_version: &u32) -> Self {
        let http_client = httper::client::HttperClient::new();

        let mut headers = HashMap::new();
        headers.insert("X-Accept".to_string(), "application/json".to_string());
        headers.insert(
            "Content-Type".to_string(),
            "application/json; charset=UTF8".to_string(),
        );

        PocketClient {
            api_key: api_key.to_string(),
            api_version: *api_version,
            http_client,
            headers,
            code: None,
        }
    }

    pub fn sign_in_step_1<'a>(
        &self,
        redirect_uri: String,
    ) -> impl Future<Item = Step1, Error = Error> + 'a {
        let url = self.get_url("oauth/request", None);

        let headers = self.headers.clone();

        let payload = SignInParams {
            consumer_key: self.api_key.to_string(),
            redirect_uri: Some(redirect_uri.to_string()),
            ..SignInParams::default()
        };

        let payload = serde_json::to_string(&payload).unwrap();

        let code = self.http_client
            .post(&url)
            .payload(payload)
            .headers(headers)
            .send()
            .json::<Code>();

        code.map(move |code| {
            let url = format!(
                "{base_url}/auth/authorize?request_token={request_token}&redirect_uri={redirect_uri}",
                base_url = BASE_URL,
                request_token = &code.code,
                redirect_uri = redirect_uri
            );

            Step1 {
                code: code.code,
                url,
            }
        })
    }

    pub fn sign_in_step_2(&self, code: String) -> impl Future<Item = User, Error = Error> {
        let url = self.get_url("oauth/authorize", None);
        let headers = self.headers.clone();

        let payload = SignInParams {
            consumer_key: self.api_key.to_string(),
            code: Some(code),
            ..SignInParams::default()
        };

        let payload = serde_json::to_string(&payload).unwrap();

        println!("Payload {:?}", payload);
        println!("Url {:?}", url);

        self.http_client
            .post(&url)
            .payload(payload)
            .headers(headers)
            .send()
            .json::<User>()
    }

    pub fn retrieve(&self) -> httper::client::response_future::ResponseFuture {
        let url = format!(
            "{base_url}/v{version}/{path}",
            base_url = BASE_URL,
            version = self.api_version,
            path = "get"
        );

        let payload = RetriveParams {
            consumer_key: self.api_key.to_string(),
            access_token: "".to_string(),
            count: 10,
        };

        let payload_as_json = serde_json::to_string(&payload);

        let data = self.http_client
            .post(&url)
            .payload(payload_as_json.unwrap())
            .send();
        data
    }

    pub fn is_signed_in(&self) -> bool {
        self.code.is_some()
    }

    fn get_url(&self, path: &str, access_token: Option<&str>) -> String {
        let url = format!(
            "{}/v{}/{}?consumer_key={token}",
            BASE_URL,
            self.api_version,
            path,
            token = self.api_key
        );

        match access_token {
            Some(access_token) => format!("{}&access_token={}", url, access_token),
            None => url,
        }
    }
}
