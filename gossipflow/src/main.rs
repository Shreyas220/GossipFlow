mod models;
use crate::models::NodeInfo;


fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <bind_address> [seed_address...]", args[0]);
        std::process::exit(1);
    }
    
    // let binder_addr = Box::new(args[1].to_string());
    // let seed_nodes =Box::new(args[2..].to_vec());
    let bind_addr = args[1].clone();
    let seed_nodes = &args[2..];

    //me here represent this node 
    let me = NodeInfo{
        address: bind_addr,
        heartbeat: 0,
    };

    //create node state 

    //handle incoming data from user and from more nodes 

    //

}
