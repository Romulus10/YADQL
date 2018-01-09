use blockchain::blockchain::Blockchain;
use core::keywords::YADQL;
use crypt::crypt::Crypt;
use regex::{Regex, Captures};

pub struct Stream {
    blockchain: Blockchain,
}

impl Stream {
    pub fn new(provider: &str) -> Stream {
        let blockchain = Blockchain::new(provider);

        Stream {
            blockchain,
        }
    }

    pub fn send(&mut self, operation: YADQL, key: &str, value: &str) {
        //! ## send(operation: &str, key: &str, value: &str)
        //! Applies transactions being sent from this machine.
        //! It was late when I wrote this... needs fixing bad.
        let res = match operation {
            YADQL::Insert(ref k, ref v) => {
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let gpgkey = c.get_key();
                self.blockchain.insert(&gpgkey, key, value);
                let payload = format!("('operation': 'insert', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            },
            YADQL::Delete(ref k) => {
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let gpgkey = c.get_key();
                self.blockchain.delete(&gpgkey, key);
                let payload = format!("('operation': 'delete', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            },
            YADQL::Update(ref k, ref v) => {
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let gpgkey = c.get_key();
                self.blockchain.update(&gpgkey, key, value);
                let payload = format!("('operation': 'update', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            },
            YADQL::Read(ref k) => {
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let gpgkey = c.get_key();
                self.blockchain.read(&gpgkey, key);
            },
            _ => panic!("I don't even know")
        };
    }
    
    pub fn recv(&self) {
        //! ## recv()
        //! Applies transactions downloaded to this machine.
        let next_query = String::new(); // This is a placeholder. Make sure we get this one from the EVM.
        let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
        let payload = c.decrypt(c.verify(next_query));
        let r = Regex::new(r"`\('operation': '(.{6})', key: '(.+)', value: '(.*)' \)`").unwrap();
        let re = r.captures(&payload).unwrap();
        let ret = match re.get(1) {
            YADQL::Insert(r.get(2), r.get(3)) => {
                insert(r.get(2), r.get(3));
            },
            YADQL::Delete(r.get(2)) => {
                delete(r.get(2));
            },
            YADQL::Update(r.get(2), r.get(3)) => {
                update(r.get(2), r.get(3));
            },
        };
    }
}
