use rspc_macros::rspc_client;

#[rspc_client((rr, dd), "liter", a = 3, rpc1, rpc2, stream rpc3)]
struct RspcClient {}

fn main() {
    println!("hello world");
}
