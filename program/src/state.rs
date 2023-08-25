use borsh::{
    BorshSerialize,
    BorshDeserialize
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NoteAccountState {
    pub is_initialized: bool,
    pub id: u16,
    pub title: String,
    pub body: String
}