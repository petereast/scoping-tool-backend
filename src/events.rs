use mpsc::SyncSender;

// System Events
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StartNewSessionEvent {
    pub session_id: String,
    pub session_title: String,
    pub session_description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndSessionEvent {
    pub session_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubmitResponseEvent {
    pub session_id: String,
    pub name: String,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub struct GetSessionDetails {
    pub session_id: String,
    pub responder: SyncSender<Result<GetSessionDetailsResponse, ()>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSessionDetailsResponse {
    pub title: String,
    pub description: String,
    pub is_ended: bool,
}

#[derive(Debug, Clone)]
pub struct GetResponseCount {
    pub session_id: String,
    pub responder: SyncSender<Result<GetResponseCountResponse, ()>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetResponseCountResponse {
    pub names: Vec<String>,
    pub is_ended: bool,
}

#[derive(Debug, Clone)]
pub struct GetSessionResult {
    pub session_id: String,
    pub responder: SyncSender<Result<GetSessionResultResponse, ()>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct _GetSessionResult {
    pub session_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSessionResultResponse {
    pub title: String,
    pub description: String,
    pub response_count: usize,
    pub average_response: u32,
    pub responses: Vec<SubmissionContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionContent {
    pub name: String,
    pub value: u32,
}

#[derive(Clone, Debug)]
pub enum SystemEvents {
    StartNewSessionEvent(StartNewSessionEvent),
    EndSessionEvent(EndSessionEvent),
    SubmitResponseEvent(SubmitResponseEvent),
    GetSessionDetails(GetSessionDetails),
    GetResponseCount(GetResponseCount),
    GetSessionResult(GetSessionResult),
}
