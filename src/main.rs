use std::env::args;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

async fn player() -> Result<()> {
    let client = reqwest::Client::new();

    let args: Vec<_> = args().collect();

    let req = client
        .get(format!("https://api.truckersmp.com/v2/player/{}", &args[2]))
        .send()
        .await?
        .text()
        .await?;

    let id = ajson::get(&req, "response.id").unwrap().unwrap();
    let name = ajson::get(&req, "response.name").unwrap().unwrap();
    let joindate = ajson::get(&req, "response.joinDate").unwrap().unwrap();
    let steamid = ajson::get(&req, "response.steamID").unwrap().unwrap();
    let groupname = ajson::get(&req, "response.groupName").unwrap().unwrap();
    let banned = ajson::get(&req, "response.banned").unwrap().unwrap();

    println!("Player ID: {}\nPlayer name: {}\nPlayer Joindate: {}\nPlayer SteamID: {}\nPlayer Group name: {}\nPlayer banned: {}", 
        id.to_string(), 
        name.to_string(), 
        joindate.to_string(), 
        steamid.to_string(), 
        groupname.to_string(), 
        banned.to_string());

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = args().collect();

    if &args[1] == "-p" {
        let _ = player().await;
    } else if &args[1] == "-v" {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
    } else {
        println!("no arg given")
    }
}
