#[macro_use]
extern crate serde_derive;


mod blockchain {
	extern crate time;
	extern crate serde;
	extern crate serde_json;
	extern crate sha2;

	use self::sha2::{Sha256, Digest};
	use std::fmt::Write;

	#[derive(Debug, Clone, Serialize)]
	struct Transaction{
		sender: String,
        recipient: String,
        amount: f32,
	}

	#[derive(Serialize, Debug)]
	pub struct BlockHeader{
	    timestamp: i64,
	    nonce: u32,
	    previous_hash: String,
	    merkle_root: String,
	    difficulty: u32
	}

	#[derive(Serialize, Debug)]
	pub struct Block{
		block_header: BlockHeader,
		transaction_count: u32,
		transactions: Vec<Transaction>
	}

	pub struct Chain {
		chain: Vec<Block>,
		current_transactions : Vec<Transaction>,
		difficulty: u32,
		miner_address: String,
		_secret: ()
	}

	impl Chain {
		pub fn new() -> Chain {
			let mut chain = Chain{ 
				chain: Vec::new(),
				current_transactions : Vec::new(),
				difficulty: 2,
				miner_address : String::from("3EhLZarJUNSfV6TWMZY1Nh5mi3FMsdHa5U"),
				_secret: ()
			};

			chain.add_reward();
			chain.generate_new_block();
			chain
		}

		pub fn add_reward(&mut self) {
			self.current_transactions = vec![];

			let transaction = Transaction {
				sender: String::new(),
		        recipient: self.miner_address.clone(),
		        amount: 12.5
			};

			self.current_transactions.push(transaction);
		}

		pub fn new_transaction(&mut self, sender: String, recipient: String, amount: f32) {
			self.current_transactions.push(Transaction {
				sender: sender,
		        recipient: recipient,
		        amount: amount,
			});

			// self.chain.last().index + 1
		}

		pub fn last_hash(&self) -> String {
			let block = match self.chain.last() {
				Some(block) => block,
				None => return String::from_utf8(vec![48; 32]).unwrap()
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
				merkle_root: String::new(),
				difficulty: self.difficulty
			};

			//merkle root hash
			block_header.merkle_root = Chain::get_merkle_root(self.current_transactions.clone());

			// add proof of work
			Chain::proof_of_work(&mut block_header);

			let block = Block{
	            block_header: block_header,
				transaction_count: self.current_transactions.len().count_ones(),
				transactions: self.current_transactions.clone()
        	};

	        self.add_reward();
	        println!("{:?}", &block);
	        self.chain.push(block);
	        &(self.chain.last().unwrap())
		}

		fn get_merkle_root(current_transactions: Vec<Transaction>) -> String {
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
				hash1.push_str(&mut hash2);
				let new_hash = Chain::hash(&hash1);
				merkle.push(new_hash);
			}

			merkle.pop().unwrap()
		}

		pub fn proof_of_work(block_header: &mut BlockHeader) {
			//pow based on difficulty
			loop {
				let hash = Chain::hash(block_header);
				let slice = &hash[..block_header.difficulty as usize];
				println!("{}", slice);
				match slice.parse::<u32>() {
					Ok(val) => {
						if val != 0 { block_header.nonce+=1; } else { break; }
					},
					Err(_) => {
						block_header.nonce+=1;
						continue;
					}
				};
				
			}

		}

		pub fn hash<T: serde::Serialize>(item: &T) -> String {
			//serialize
			let input = serde_json::to_string(&item).unwrap();
			let mut hasher = Sha256::default();
			hasher.input(input.as_bytes());
			// hasher.result()
			// println!("Result: {:x}", hasher.result());
			let result = hasher.result();
			let vec_result = result.to_vec();

			Chain::hex_to_string(vec_result.as_slice())
		}

		pub fn hex_to_string(vec_result: &[u8]) -> String {
			let mut s = String::new();
		    for byte in vec_result {
		        write!(&mut s, "{:x}", byte).expect("Unable to write");
		    }
		    s
		}
	}
}


fn main() {
	let chain = blockchain::Chain::new();
}