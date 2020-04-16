use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
#[derive(BorshSerialize,BorshDeserialize,PartialEq,Clone,Serialize,Deserialize)]
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
#[derive(Default,BorshSerialize,BorshDeserialize,Serialize,Deserialize,Clone)]
pub struct VoteInfo{
    pub voting_powers : u128,
    pub timestamp : u64,
    pub block_index : u64,
    pub weight : u128
}

#[derive(BorshSerialize,BorshDeserialize,Clone,Serialize,Deserialize)]
pub struct Voter{
    pub has_voted: bool,
    pub vote : bool,
    pub weight : u128,
    pub voting_powers : u128,
    pub vote_infos : Vec<VoteInfo>
}

impl Default for Voter {
    fn default() -> Self{
        Voter {has_voted : true,
        vote:true,
        weight:0,
        voting_powers:0,
        vote_infos:Vec::new()}
    }
}
#[derive(Default,BorshSerialize,BorshDeserialize,Clone,Serialize,Deserialize)]
pub struct Proposal{
    pub creator :String,
    pub status :ProposalStatus,
    pub yes_votes:u128,
    pub no_votes:u128,
    pub name:String,
    pub description:String,
    pub voters : HashMap<String,Voter>,
    pub expiration_time:u64,
    pub block_index:u64
}