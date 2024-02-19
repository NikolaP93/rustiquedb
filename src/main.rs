mod server;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let server_handle = tokio::spawn(server::run());

  let client_handle = tokio::spawn(client::run());

  let server_result = server_handle.await?;
  let client_result = client_handle.await?;

  println!("Server result: {:?}", server_result);
  println!("Client result: {:?}", client_result);

  Ok(())
}