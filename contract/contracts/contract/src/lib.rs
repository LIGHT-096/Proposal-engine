#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, Address};

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub title: Symbol,
    pub description: Symbol,
    pub creator: Address,
    pub votes_for: u32,
    pub votes_against: u32,
}

#[contract]
pub struct ProposalEngine;

#[contractimpl]
impl ProposalEngine {
    // Create a new proposal
    pub fn create_proposal(
        env: Env,
        id: u32,
        title: Symbol,
        description: Symbol,
        creator: Address,
    ) {
        let proposal = Proposal {
            id,
            title,
            description,
            creator,
            votes_for: 0,
            votes_against: 0,
        };

        env.storage().instance().set(&id, &proposal);
    }

    // Vote on a proposal
    pub fn vote(env: Env, id: u32, support: bool) {
        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&id)
            .expect("Proposal not found");

        if support {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }

        env.storage().instance().set(&id, &proposal);
    }

    // Get proposal details
    pub fn get_proposal(env: Env, id: u32) -> Proposal {
        env.storage()
            .instance()
            .get(&id)
            .expect("Proposal not found")
    }
}


