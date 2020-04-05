use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{
    env,
    ext_contract,
    near_bindgen,
    Promise,
};
use serde_json::json;
pub mod proposal;
use crate::proposal::{Proposal, ProposalStatus, Voter};
use near_bindgen::env::block_timestamp;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! only_owner{
    ($s:ident) => {assert!($s.owner == env::signer_account_id(),"only owner can call this method");};
}
#[near_bindgen]
#[derive(BorshSerialize,BorshDeserialize)]
pub struct QVVoting{
    owner : String,
    total_supply : u128,
    symbol: String,
    name : String,
    balances : HashMap<String,u128>,
    proposals : Vec<Proposal>,
    create_cost : u128
}


impl Default for QVVoting{
    fn default() -> Self{
        QVVoting {name : "QV Voting".to_owned(),
            symbol : "QVV".to_owned(),
            create_cost : 100,
            owner: "shellteo".to_owned(),
            balances:HashMap::new(),
            proposals:Vec::new(),
            total_supply:0
        }
    }
}

#[near_bindgen]
impl QVVoting{
    pub fn create_proposal(&mut self,name:String,description:String,
                           expiration_time:u64) -> usize{
        assert!(expiration_time > 0, "The voting period cannot be 0");
        let cost = self.create_cost;
        let sender : String = env::signer_account_id();
        let money=self.get_balance_mut(sender.clone());
        assert!(*money >= cost, "Do not have enough balance to create a proposal");
        *money-=cost;
        self.proposals.push(Proposal {
            creator : sender,
            status : ProposalStatus::IN_PROGRESS,
            name,description,
            expiration_time:expiration_time*1000000000 + env::block_timestamp(),
            ..Default::default()
        });
        self.proposals.len()-1
    }
    pub fn set_proposal_to_tally(&mut self,proposal_id : usize) {
        only_owner!(self);
        assert!(self.proposals[proposal_id].status == ProposalStatus :: IN_PROGRESS,
        "Vote is not in progress");
        assert!(env::block_timestamp() >=
            self.proposals[proposal_id].expiration_time,
                "voting period has not expired");
        self.proposals[proposal_id].status = ProposalStatus::TALLY;
    }
    pub fn set_proposal_to_ended(&mut self,proposal_id:usize){
        assert!(self.proposals[proposal_id].status == ProposalStatus :: TALLY,
                "Vote is not in tally");
        assert!(env::block_timestamp() >=
                    self.proposals[proposal_id].expiration_time,
                "voting period has not expired");
        self.proposals[proposal_id].status = ProposalStatus::ENDED;
    }
    pub fn get_create_cost(&self) -> u128 { self.create_cost }
    pub fn get_proposal_count(&self) -> usize {self.proposals.len()}
    pub fn get_total_supply(&self) -> u128 {self.total_supply}
    pub fn get_proposal(&self,proposal_id:usize) -> Proposal {
        self.proposals[proposal_id].clone()}
    pub fn set_create_cost(&mut self,cost : u128){
        only_owner!(self);
        self.create_cost = cost;
    }
    pub fn get_proposal_status(&self,proposal_id:usize) -> ProposalStatus{
        self.proposals[proposal_id].status.clone()
    }
    pub fn get_proposal_expiration_time(&self,proposal_id:usize) ->u64{
        self.proposals[proposal_id].expiration_time
    }
    pub fn count_votes(&self,proposal_id:usize) -> (u128,u128) {
        let mut yes_votes = 0u128;
        let mut no_votes = 0u128;

        for (_,v) in self.proposals[proposal_id].voters.iter() {
            let weight = v.weight;
            if v.vote{
                yes_votes+=weight;
            }else{
                no_votes+=weight;
            }
        }
        (yes_votes,no_votes)
    }
    pub fn cast_vote(&mut self,proposal_id:usize,num_tokens:u128,vote:bool){
        assert!(self.proposals[proposal_id].status==ProposalStatus::IN_PROGRESS,
        "proposal has expired.");
        assert!(self.proposals[proposal_id].expiration_time >= env::block_timestamp(),
        "for this proposal, the voting time expired");
        let sender = env::signer_account_id();
        assert!(!self.user_has_voted(&sender,proposal_id),
                "user already voted on this proposal");
        let balance = self.get_balance_mut(sender.clone());
        assert!(*balance>=num_tokens,"do not have enough money to vote");
        *balance-=num_tokens;
        let weight= f64::sqrt(num_tokens as f64) as u128;
        self.proposals[proposal_id].voters.insert(sender,Voter {
            has_voted : true,
            vote,
            weight
        });
        if vote{
            self.proposals[proposal_id].yes_votes+=weight;}
        else {
            self.proposals[proposal_id].no_votes+=weight;
        }

    }
    fn user_has_voted(&self,name:&str,proposal_id:usize) -> bool{
        for k in self.proposals[proposal_id].voters.keys(){
            if k == name { return true}
        }
        false
    }
    pub fn mint(&mut self,name : String,amount:u128) {
        only_owner!(self);
        self.total_supply += amount;
        *(self.get_balance_mut(name))+=amount;
    }
    pub fn balance_of(&self,name : String) -> u128{
        self.balances.get(&name).unwrap_or(&0).clone()
    }
    fn get_balance_mut(&mut self,name : String) ->&mut u128{
        self.balances.entry(name).or_insert(0)
    }

}
