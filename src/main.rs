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
	pub struct Block{
	    index: u32,
	    timestamp: i64,
	    transactions: Vec<Transaction>,
	    proof: u32,
	    previous_hash: u32
	}

	pub struct Chain {
		chain: Vec<Block>,
		current_transactions : Vec<Transaction> ,
		_secret: ()
	}

	impl Chain {
		pub fn new() -> Chain {
			let mut chain = Chain{ 
				chain: Vec::new(),
				current_transactions : Vec::new(),
				_secret: ()
			};

			chain.new_block(100, 1);
			chain
		}

		pub fn new_transaction(&mut self, sender: String, recipient: String, amount: u32) -> u32 {
			self.current_transactions.push(Transaction {
				sender: sender,
		        recipient: recipient,
		        amount: amount,
			});

			// self.chain.last().index + 1
			match self.chain.last() {
				Some(block) => block.index + 1,
				None => 0
			}
		}

		pub fn new_block(&mut self, proof: u32, previous_hash: u32) -> &Block {
			let block = Block{
	            index: self.chain.len().count_ones() + 1,
	            timestamp: time::now().to_timespec().sec,
	            transactions: self.current_transactions.clone(),
	            proof: proof,
	            previous_hash: previous_hash,
        	};

	        self.current_transactions = vec![];
	        self.chain.push(block);
	        &(self.chain.last().unwrap())
		}

		pub fn test(&self) {
			self.chain.last().unwrap().hash()
		}
	}

	impl Block{
		pub fn hash(&self){
			//serialize
			let input = serde_json::to_string(&self).unwrap();
			let mut hasher = Sha256::default();
			hasher.input(input.as_bytes());
			// hasher.result()
			println!("Result: {:x}", hasher.result());
		}
	}
}


fn main() {
	let chain = blockchain::Chain::new();
	chain.test()

}