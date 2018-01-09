use web3;
use ethabi;
use blockchain::err::BlockchainError;
use regex::Regex;
use crypt::crypt::Crypt;
use core::keywords::YADQL;
use parser::parser::Parser;
use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::transports::http::Http;
use web3::types::{Address, U256};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct KeyVal {
    /// # KeyVal
    pub key: String,
    pub val: String
}

pub struct Blockchain {
    /// # struct Blockchain
    pub memory: Vec<KeyVal>,
    provider: &str,
    cAddr: Address,
    ethabi: ethabi::Contract,
}

pub struct Success {
    /// # Success
    pub payload: String
}

fn init_web3(provider: &str) -> web3::Web3<web3::transports::Http> {
    let (_eloop, transport) = web3::transports::Http::new(provider).unwrap(); 
    let web3 = web3::Web3::new(transport); 
    let accounts = web3.eth().accounts().wait().unwrap();
    web3
}

impl Blockchain {
    //! # impl Blockchain
    //! Represents the current state of the blockchain. This honestly shouldn't care if we're submitting or receiving the queries; it should be able to handle upload as well as download. 
    pub fn new(provider: &str) -> Blockchain {
        let mut file = File::open("contract_address").unwrap();
        let mut contents = String::new();
        let addrutf8: String = file.read_to_string(&mut contents).unwrap().to_string();
        let cAddr: Address = addrutf8.parse().unwrap();
        let json: &[u8] = include_bytes!("./../compiled/yadql.abi");
        let ethabiC = ethabi::Contract::load(json).unwrap();
        // let storg_contract = web3::contract::Contract::new(web3.eth(), cAddr, ethabiC);
        
        Blockchain {
            provider,
            cAddr,
            ethabi: ethabiC,
            memory: Vec::new()
        }
    }

    pub fn insert(&mut self, pubkey: &str, key: &str, value: &str) -> Result<Success, BlockchainError> {
        //! ## insert(key: &str, value: &str) -> Result<Success, BlockchainError>
        //! Inserts a new value into the blockchain.
        
        //! Should throw an error if the record already exists.
        self.memory.push(KeyVal { key: key.to_string(), val: value.to_string() });
        let web3 = init_web3(self.provider);
        let storg = web3::contract::Contract::new(web3.eth(), self.cAddr, self.ethabi);
        storg.query("store", (calculate_hash(pubkey), key, value), None, Options::default(), None);
        Ok(Success { payload: self.memory[self.memory.len() - 1].val.clone() })
    }

    pub fn delete(&mut self, pubkey: &str, key: &str) -> Result<Success, BlockchainError>{
        //! ## delete(key: &str) -> Result<Success, BlockchainError>
        //! Marks a record as deleted.
        //! Should fail if the record doesn't exist.
        let ind = self.memory.iter().position(|ref r| r.key == key).unwrap();
        self.memory.remove(ind);
        let web3 = init_web3(self.provider);
        let storg = web3::contract::Contract::new(web3.eth(), self.cAddr, self.ethabi);
        storg.query("store", (calculate_hash(pubkey), key, ""), None, Options::default(), None);
        Ok(Success { payload: "".to_string() })
    }

    pub fn update(&mut self, pubkey: &str, key: &str, value: &str) -> Result<Success, BlockchainError> {
        //! ## update(key: &str, value: &str) -> Result<Success, BlockchainError>
        //! Updates a value in the local database and submits to the blockchain.
        //! Should fail if the record doesn't exist.
        let mut it = self.memory.iter();
        let mut ret;
        'update: loop {
            let curr = it.next().unwrap();
            if curr.key == key.to_string() {
                ret = Success { payload: curr.key.clone() };
                break 'update;
            }
        }
        let web3 = init_web3(self.provider);
        let storg = web3::contract::Contract::new(web3.eth(), self.cAddr, self.ethabi);
        storg.query("store", (calculate_hash(pubkey), key, value), None, Options::default(), None);

        Ok(ret)
    }

    pub fn read(&self, pubkey: &str, key: &str) -> Result<Success, BlockchainError> {
        //! ## read(key: &str) -> Result<Success, BlockchainError>
        //! A method on the Blockchain struct that reads data from memory into the application's context for use. This really just worries about the local mirror of the database; we really don't care about what the blockchain looks like.
        //! Should fail if the record doesn't exist.
        let mut it = self.memory.iter();
        let ret;
        'read: loop {
            let curr = it.next().unwrap();
            if curr.key == key.to_string() {
                ret = Success { payload: curr.val.clone() };
                break 'read;
            }
        }
        Ok (ret)
    }
}
