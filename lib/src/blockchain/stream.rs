use blockchain::blockchain::Blockchain;
use core::keywords::YADQL;
use crypt::crypt::Crypt;
use regex::{Regex, Captures};

const gpgkey: &'static str = "mQENBFpRx2sBCADR3alAVTA8tsar5F/coV+m1zQOKPGCVG5KPs3nGwMQIhvtaosp69WvPrctuUbljNZsHbXq0Iuvu4fq8Oa7r5ZhKgUI0qSG/8vu5l057QNHXQwVZhH1AbEEcwrxS6KUcqiBbZg76Pl8cilWViXoaZwNg72C8M5xtizGG0JZm7ibcGbrbWBGyPsi5Mosb9qhLiIqYQoNvWvibwx3viXL5wzTjFPDh6x2nr1/99iKpipJobkSep6oDRWPQmIb6gvJXoqT4DtCggAx77hJ7s0hjnX6MzMx3aYoAsB2VAvSn0KzsAP/sgMtHteM3GaWZ+sm7CrGigDxoCHpALlZKnidIvAZABEBAAG0NVRlc3RpbmcgKE5vdCBhIHJlYWwga2V5cGFpci4pIDx0ZXN0QHJhZGljYWwteWFkcWwuaW8+iQE3BBMBCAAhBQJaUcdrAhsDBQsJCAcCBhUICQoLAgQWAgMBAh4BAheAAAoJEHuO6pJIxZ2QU/gIAK3hyGSK5GpLvmOlNBVh96HMu9P3Ha5EHrXLBe/lQ/Cxtg/XTQxHnpLs+MtUltrHMNg5QGBd2cajtZaRZbN7XepZcnUl95GAZRG1hMq+C5pyyx3nbVXnDCoorSu3IVRHscZSA6uqN0DdJW06bYxDroyQOA3l8Rg+iMON5iVk4rPqO2sjad6xDwppnLmrq85jDkk2lCe1rgHLfPwjCuqF0q3o3e7MLKnXktXDNhV54FAui7nQtoRt9UuoBK4LL6oI2lEHR7UVyZfBDVtfdow4lqQEG4yNOhEnmmnaKUdg/RTlfHZbwJkTP/kfW+EIfFPy+YdnzMsq4vlcxY+XQAnhsLC5AQ0EWlHHawEIAMLXgbGXMpbDBheAT/5Wp3A7GgOjv4f/Odo2NTIb+Ny9e9u6+Qf1mzbMCRfpfUH7wfWlJgRzOLjI4ODyF3Hqw74kMIltSkT+EvmsRwetIYGCSKDVnaj8627YG0azeAGQo8u7zfcIqSjegDVze8othcIOlk/f5ysR7MUtqOsvhWDifDmbU8O8w1B85ROyC/3tHwpbzQnIiBR6u7k2rDsHnypMqYk9nJ8TVgFXOHkR3eoZMSyIRzQLkYaznKmI5qFMr0vgqPqQ+QsByLhQ7SZHqBEhjJc74Pxjt/eMMFvEjUgEdzEKK8fDJwQHdIfwsdHO6/Bk5vss+VlimQAUwp3Q8+cAEQEAAYkBHwQYAQgACQUCWlHHawIbDAAKCRB7juqSSMWdkI+AB/40Zx/q+/vzNT6F5QbvcJfIUKBMJ+fVJayd36ErGG3eF1W49DB2TlykbaYdhHlskwUu3wQHqQ5vXOLhM71Feb06VoXZf/vfH/vbcCSr7zYGJcx3gNiCb7Jo+i/5BVjR9rM3HwzO0UAgabuUpQPiePWWCkQeJ/CZY5Yw39C9qnhfRikIhVdul557ZJiBosUbvHWMGre27El/p1z68PFkZfAGMtahDsjhaVMgGoJWVfouwfjKKDj90PPA0sPGN2B6w/l2BUAvZwtFGTbnBOOXfPOJCynpgEyU2O+zUwv8UkMl53Yrd2tZiDHYjO6sz25bTkSRf20Xu7s6hnaUIP+CTZYK=rBF1";


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
                self.blockchain.insert(gpgkey, key, value);
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let payload = format!("('operation': 'insert', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            },
            YADQL::Delete(ref k) => {
                self.blockchain.delete(gpgkey, key);
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let payload = format!("('operation': 'delete', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            },
            YADQL::Update(ref k, ref v) => {
                self.blockchain.update(gpgkey, key, value);
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let payload = format!("('operation': 'update', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            },
            YADQL::Read(ref k) => {
                self.blockchain.read(key);
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
