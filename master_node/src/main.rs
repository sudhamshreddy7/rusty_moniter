use std::io;
mod utils::get_logs;
pub struct Node{
    ip:String,
    port:String,
    authorization_string:String,
}
#[tokio::main]
async fn main() {
    let mut input = String::new();
    println!("Enter the number of nodes that you want to moniter");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_nodes: i32 = input.trim().parse().expect("Please enter a valid number");
    let mut nodes: Vec<Node> = Vec::new();
    fill(num_nodes, &mut nodes);
    let addr:std::net::SocketAddr = "0.0.0.0:8777".parse().unwrap();


}
fn fill(num_nodes:i32, nodes: &mut Vec<Node>) {
    while nodes.len() < num_nodes as usize {
        let mut input   = String::new();
        println!("Enter valid IP:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let ip = input.trim().to_string();

        println!("Enter Port number:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let port = input.trim().to_string();
        println!("Enter authorization key:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let authorization_key = input.trim().to_string();

        nodes.push(Node {
            ip:ip.to_string(),
            port:port,
            authorization_string:authorization_key,
        });
    }
}