use std::io::{self, BufRead, Write};

use dcsjsonrpc_client::{Client, Error};
use serde_json::Value;

fn main() -> Result<(), Error> {
    let client = Client::connect("127.0.0.1:7777")?;

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        if let Some(line) = lines.next() {
            let result: Value = match client.execute(&line?) {
                Ok(v) => v,
                Err(Error::Rpc(err)) => {
                    eprintln!("{}", err);
                    continue;
                }
                Err(err) => return Err(err),
            };
            let json = serde_json::to_string_pretty(&result)?;
            println!("= {}", json);
        }
    }
}
