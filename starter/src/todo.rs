use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Default, CandidType, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub text: String,
}

