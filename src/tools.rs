use web3::contract::Contract;
use web3::transports::Http;
use web3::types::Address;
use web3::Web3;

pub fn erc20_at(address: Address, ronin: &Web3<Http>) -> web3::ethabi::Result<Contract<Http>> {
    Contract::from_json(ronin.eth(), address, include_bytes!("./abi/erc20_min.json"))
}
