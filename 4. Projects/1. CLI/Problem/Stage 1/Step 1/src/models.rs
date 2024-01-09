use std::collections::HashMap;

pub enum Status {
    // TODO: add fields (make sure the fields are public)
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    // TODO: add fields (make sure the fields are public)
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open and the stories should be an empty vector
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![]
        }
    }
}

pub struct Story {
    // TODO: add fields (make sure the fields are public)
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

// This struct represents the entire db state which includes the last_item_id, epics, and stories
pub struct DBState {
    // TODO: add fields (make sure the fields are public)
    pub last_item_id: String,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}