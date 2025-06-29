use poem::{
    Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
};
use solana_client::rpc_client::RpcClient;
use solana_program::{hash::hash, pubkey::Pubkey, system_instruction::transfer};

use solana_sdk::{
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
const RPC_URL: &str = "https://api.devnet.solana.com";
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct GetBalance {
    pub wallet: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBalanceRespose {
    pub wallet_add: String,
    pub balance_lamports: u64,
    pub balance_sol: f64,
}

#[handler]
fn get_balance(Json(data): Json<GetBalance>) -> Json<GetBalanceRespose> {
    let wallet_add = data.wallet;
    let client = RpcClient::new(RPC_URL);
    let wallet = wallet_add.parse::<Pubkey>().unwrap();
    let balance = client.get_balance(&wallet).unwrap();
    Json(GetBalanceRespose {
        wallet_add,
        balance_lamports: balance,
        balance_sol: balance as f64 / 1_000_000_000.0,
    })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/get_balance", get(get_balance));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
