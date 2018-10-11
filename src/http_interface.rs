#[derive(Serialize, Deserialize, Debug)]
pub struct NewScopingSessionCmd {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndSessionCmd {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitResponseCmd {
    pub session_id: String,
    pub name: String,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSessionDetailsOkResponse {
    pub session_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct GetSessionDetailsCmd {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewScopingSessionOkResponse {
    pub id: String,
    pub url: String,
    pub expires_at: String,
}

impl NewScopingSessionOkResponse {
    pub fn new(id: String, url: String, expires_at: String) -> Self {
        Self {
            id,
            url,
            expires_at,
        }
    }
}