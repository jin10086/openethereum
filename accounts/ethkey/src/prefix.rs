// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Open Ethereum.

// Open Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Open Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Open Ethereum.  If not, see <http://www.gnu.org/licenses/>.

use ethereum_types::{Address, H256, U256};
use keccak_hash::keccak;
use parity_crypto::publickey::{Error, Generator, KeyPair, Random};
use rlp::RlpStream;

/// Tries to find keypair with address starting with given prefix.
pub struct Prefix {
	prefix: Vec<u8>,
	iterations: usize,
}

impl Prefix {
	pub fn new(prefix: Vec<u8>, iterations: usize) -> Self {
		Prefix { prefix, iterations }
	}

	pub fn generate(&mut self) -> Result<KeyPair, Error> {
		for _ in 0..self.iterations {
			let keypair = Random.generate();
			let nonce = U256::zero();
			let sender = keypair.address();
			let mut stream = RlpStream::new_list(2);
			stream.append(&sender);
			stream.append(&nonce);
			let contract_address = From::from(keccak(stream.as_raw()));
			if contract_address.as_bytes().starts_with(&self.prefix) {
				return Ok(keypair);
			}
		}

		Err(Error::Custom("Could not find keypair".into()))
	}
	pub fn mk_contract_address(sender: Address, nonce: U256) -> Address {
		let mut stream = RlpStream::new_list(2);
		stream.append(&sender);
		stream.append(&nonce);
		From::from(keccak(stream.as_raw()))
	}
}

#[cfg(test)]
mod tests {
	use Prefix;

	#[test]
	fn prefix_generator() {
		let prefix = vec![0xffu8];
		let keypair = Prefix::new(prefix.clone(), usize::max_value())
			.generate()
			.unwrap();
		assert!(keypair.address().as_bytes().starts_with(&prefix));
	}
}
