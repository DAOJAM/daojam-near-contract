use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
#[derive(BorshSerialize,BorshDeserialize,PartialEq,Clone)]
pub enum ProposalStatus{
    IN_PROGRESS,
    TALLY,
    ENDED
}

impl Default for ProposalStatus {
    fn default() -> Self{
        ProposalStatus::IN_PROGRESS
    }
}

impl Into<String> for ProposalStatus {
    fn into(self) -> String {
        match self{
            ProposalStatus::IN_PROGRESS => "IN_PROGRESS",
            ProposalStatus::TALLY => "TALLY",
            ProposalStatus::ENDED => "ENDED"
        }.to_owned()
    }
}

#[derive(Default,BorshSerialize,BorshDeserialize)]
pub struct Voter{
    pub has_voted: bool,
    pub vote : bool,
    pub weight : u128
}
#[derive(Default,BorshSerialize,BorshDeserialize)]
pub struct Proposal{
    pub creator :String,
    pub status :ProposalStatus,
    pub yes_votes:u128,
    pub no_votes:u128,
    pub name:String,
    pub description:String,
    pub voters : HashMap<String,Voter>,
    pub expiration_time:u64
}