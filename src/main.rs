use toy_blockchain::Blockchain;

fn main() {
    println!("Initializing blockchain");
    let mut blockchain = Blockchain::new();

    println!("Adding block 1");
    blockchain.add_block("Hello World!".to_string());
    println!("Adding block 2");
    blockchain.add_block("Goodbye world!".to_string());

    println!("blockchain valid: {}", blockchain.is_chain_valid());

    println!("--- Blockchain ---\n{}", blockchain);
}
