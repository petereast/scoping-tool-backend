use events::SubmissionContent;

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
    pub is_ended: bool,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct GetSessionDetailsCmd {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetSessionResultCmd {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewScopingSessionOkResponse {
    pub id: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponseCountOkResponse {
    pub names: Vec<String>,
    pub count: usize,
    pub session_id: String,
    pub submission_url: String,
    pub is_ended: bool,
}

#[derive(Serialize, Debug)]
pub struct GetSessionResultOkResponse {
    pub title: String,
    pub description: String,
    pub response_count: usize,
    pub average_response: u32,
    pub responses: Vec<SubmissionContent>,
}

impl NewScopingSessionOkResponse {
    pub fn new(id: String, url: String) -> Self {
        Self { id, url }
    }
}
