use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_id: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lane {
    pub id: String,
    pub board_id: String,
    pub name: String,
    pub position_in_board: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub description: String,
    pub position_in_lane: u16,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardTaskItem {
    pub id: String,
    pub card_id: String,
    pub text: String,
    pub is_complete: bool,
}
