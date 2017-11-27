// extern crate time;
// use std::ptr::null;
// use std::time::SystemTime;
#[macro_use]
extern crate serde_derive;


mod blockchain {
	extern crate time;
	extern crate serde;
	extern crate serde_json;
	extern crate sha2;

	use self::sha2::{Sha256, Digest};

	#[derive(Debug, Clone, Serialize)]
	struct Transaction{
		sender: String,
        recipient: String,
        amount: u32,
	}

	#[derive(Serialize)]
	pub struct BlockHeader{
	    timestamp: i64,
	    nonce: u32,
	    previous_hash: Vec<u8>,
	    merkle_root: Vec<u8>,
	    difficulty: u32
	}

	#[derive(Serialize)]
	pub struct Block{
		block_header: BlockHeader,
		transaction_count: u32,
		transactions: Vec<Transaction>
	}

	pub struct Chain {
		chain: Vec<Block>,
		current_transactions : Vec<Transaction>,
		difficulty: u32,
		_secret: ()
	}

	impl Chain {
		pub fn new() -> Chain {
			let mut chain = Chain{ 
				chain: Vec::new(),
				current_transactions : Vec::new(),
				difficulty: 1,
				_secret: ()
			};

			chain.generate_new_block();
			chain
		}

		pub fn new_transaction(&mut self, sender: String, recipient: String, amount: u32) {
			self.current_transactions.push(Transaction {
				sender: sender,
		        recipient: recipient,
		        amount: amount,
			});

			// self.chain.last().index + 1
		}

		pub fn last_hash(&self) -> Vec<u8> {
			let block = match self.chain.last() {
				Some(block) => block,
				None => return vec![0]
			};

			Chain::hash(&block.block_header)
		}

		pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
			self.difficulty = difficulty;
			true
		}

		pub fn generate_new_block(&mut self) -> &Block {
			let mut block_header = BlockHeader{
				timestamp: time::now().to_timespec().sec,
				nonce: 0,
				previous_hash: self.last_hash(),
				merkle_root: vec![],
				difficulty: self.difficulty
			};

			//merkle root hash
			block_header.merkle_root = Chain::get_merkle_root(self.current_transactions.clone());

			// add proof of work
			

			let block = Block{
	            block_header: block_header,
				transaction_count: self.current_transactions.len().count_ones(),
				transactions: self.current_transactions.clone()
        	};

	        self.current_transactions = vec![];
	        self.chain.push(block);
	        &(self.chain.last().unwrap())
		}

		fn get_merkle_root(current_transactions: Vec<Transaction>) -> Vec<u8> {
			let mut merkle = Vec::new();

			for transaction in &current_transactions {
				let transaction_hash = Chain::hash(transaction);
				merkle.push(transaction_hash);
			}

			if merkle.len() % 2 == 1 {
				let last = merkle.last().cloned().unwrap();
				merkle.push(last);
			}

			while merkle.len() > 1 {
				let mut hash1 = merkle.pop().unwrap();
				let mut hash2 = merkle.pop().unwrap();
				hash1.append(&mut hash2);
				let new_hash = Chain::hash(&hash1);
				merkle.push(new_hash);
			}

			merkle.pop().unwrap()
		}

		pub fn hash<T: serde::Serialize>(item: &T) -> Vec<u8> {
			//serialize
			let input = serde_json::to_string(&item).unwrap();
			let mut hasher = Sha256::default();
			hasher.input(input.as_bytes());
			// hasher.result()
			// println!("Result: {:x}", hasher.result());
			hasher.result().to_vec()
		}
	}
}


fn main() {
	let chain = blockchain::Chain::new();
}