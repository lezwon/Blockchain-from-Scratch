#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

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
		reward_amount: f32,
		_secret: ()
	}

	impl Chain {
		pub fn new(miner_address: String, difficulty: u32) -> Chain {
			let mut chain = Chain{ 
				chain: Vec::new(),
				current_transactions : Vec::new(),
				difficulty: difficulty,
				miner_address : miner_address,
				reward_amount: 12.5,
				_secret: ()
			};

			chain.generate_new_block();
			chain
		}

		pub fn new_transaction(&mut self, sender: String, recipient: String, amount: f32) -> bool {
			self.current_transactions.push(Transaction {
				sender: sender,
		        recipient: recipient,
		        amount: amount,
			});

			true
		}

		pub fn last_hash(&self) -> String {
			let block = match self.chain.last() {
				Some(block) => block,
				None => return String::from_utf8(vec![48; 64]).unwrap()
			};

			Chain::hash(&block.block_header)
		}

		pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
			self.difficulty = difficulty;
			true
		}

		pub fn update_reward(&mut self, reward_amount: f32) -> bool {
			self.reward_amount = reward_amount;
			true
		}

		pub fn generate_new_block(&mut self) -> bool {
			let mut block_header = BlockHeader{
				timestamp: time::now().to_timespec().sec,
				nonce: 0,
				previous_hash: self.last_hash(),
				merkle_root: String::new(),
				difficulty: self.difficulty
			};

			let reward_transaction = Transaction {
				sender: String::new(),
		        recipient: self.miner_address.clone(),
		        amount: self.reward_amount
			};

			let mut block = Block{
	            block_header: block_header,
				transaction_count: 0,
				transactions: vec![]
        	};

        	
        	block.transactions.push(reward_transaction);
        	block.transactions.append(&mut self.current_transactions);
        	block.transaction_count = block.transactions.len() as u32;

        	//merkle root hash
			block.block_header.merkle_root = Chain::get_merkle_root(block.transactions.clone());
			//proof of work
			Chain::proof_of_work(&mut block.block_header);


	        println!("{:#?}", &block);
	        self.chain.push(block);
	        true
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
				let mut hash1 = merkle.remove(0);
				let mut hash2 = merkle.remove(0);
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
				match slice.parse::<u32>() {
					Ok(val) => {
						if val != 0 { block_header.nonce+=1; } 
						else { 
							println!("BLOCK HASH: {}", hash);
							break; 
						}
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
	let mut miner_address = String::new();
	let mut difficulty_string = String::new();
	let mut choice = String::new();

	println!("##########################################");
	println!("\t BLOCKCHAIN FROM SCRATCH ");
	println!("##########################################");

	print!("MINER ADDRESS: ");
	io::stdout().flush();
	io::stdin().read_line(&mut miner_address);
	print!("DIFFICULTY: ");
	io::stdout().flush();
	io::stdin().read_line(&mut difficulty_string);
	let difficulty: u32 = difficulty_string.trim().parse().unwrap();
	println!("++++++++++++++++ INITIATING CHAIN ++++++++++++++++++++");
	println!("::::::::::::: GENERATING GENESIS BLOCK :::::::::::::::");
	let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), difficulty);

	loop{
		println!("-----------------------------------------");
		println!("\t\tMENU ");
		println!("1. NEW TRANSACTION");
		println!("2. MINE BLOCK");
		println!("3. CHANGE DIFFICULTY LEVEL");
		println!("4. CHANGE REWARD AMOUNT");
		println!("0. EXIT");
		print!("ENTER YOUR CHOICE: ");
		io::stdout().flush();
		choice.clear();
		io::stdin().read_line(&mut choice);
		println!("-----------------------------------------");

		match choice.trim().parse().unwrap() {
			0 => 
			{
				println!("--------------------EXITING----------------------");
				process::exit(0);
			},
			1 => 
			{
				let mut sender = String::new();
				let mut recipient = String::new();
				let mut amount = String::new();

				print!("ENTER SENDER ADDRESS: ");
				io::stdout().flush();
				io::stdin().read_line(&mut sender);
				print!("ENTER RECEIPIENT ADDRESS: ");
				io::stdout().flush();
				io::stdin().read_line(&mut recipient);
				print!("ENTER AMOUNT: ");
				io::stdout().flush();
				io::stdin().read_line(&mut amount);
				let res = chain.new_transaction(sender.trim().to_string(), recipient.trim().to_string(), amount.trim().parse().unwrap());
				match res {
					true => println!("TRANSACTION ADDED SUCCESSFULLY!"),
					false => println!("TRANSACTION ADDITION FAILED!"),
				}
			},
			2 => 
			{
				println!("::::::::::::::::::: GENERATING BLOCK ::::::::::::::::::::::");
				let res = chain.generate_new_block();
				match res {
					true => println!("BLOCK GENERATED SUCCESSFULLY!"),
					false => println!("BLOCK GENERATION FAILED!"),
				}
			}
			3 => 
			{
				let mut new_difficulty = String::new();
				print!("ENTER NEW DIFFICULTY:");
				io::stdout().flush();
				io::stdin().read_line(&mut new_difficulty);
				let res = chain.update_difficulty(new_difficulty.trim().parse().unwrap());
				match res {
					true => println!("UPDATED DIFFICULTY SUCCESSFULLY!"),
					false => println!("DIFFICULTY UPDATION FAILED!"),
				}
			},
			4 => 
			{
				let mut new_reward = String::new();
				print!("ENTER NEW REWARD AMOUNT:");
				io::stdout().flush();
				io::stdin().read_line(&mut new_reward);
				let res = chain.update_reward(new_reward.trim().parse().unwrap());
				match res {
					true => println!("UPDATED REWARD AMOUNT SUCCESSFULLY!"),
					false => println!("REWARD AMOUNT UPDATION FAILED!"),
				}
			},
			_ => println!("************\t INVALID OPTION. PLEASE RETRY \t************"),
		}

	}


}