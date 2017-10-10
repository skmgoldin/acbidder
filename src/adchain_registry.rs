extern crate web3;

use std::str::FromStr;
use web3::types::*;
use web3::contract::Contract;
use web3::contract::tokens::Tokenizable;
use web3::futures::Future;

pub struct RegistryInstance<'a, T: 'a + web3::Transport> {
    instance: Contract<&'a T>
}

impl<'a, T: web3::Transport> RegistryInstance<'a, T> {

    pub fn new(web3: &'a web3::Web3<T>) -> RegistryInstance<'a, T> {
        const REGISTRY_ADDR: &str = "0x9452b742302e50a28660fd00748fa05e071f1470";

        let instance = Contract::from_json(
            web3.eth(),
            H160::from_str(REGISTRY_ADDR).unwrap(), //TODO:Make static
            include_bytes!("../Registry.json")
        ).unwrap();

        RegistryInstance {
            instance
        }
    }

    pub fn is_in_registry(&self, domain: &str) -> bool {
        let domain = String::from(domain).into_token();
        let result: bool = self.instance.query("isWhitelisted", (domain, ), None,
            web3::contract::Options::default(), BlockNumber::Latest).wait().unwrap();
        result
    }
}

