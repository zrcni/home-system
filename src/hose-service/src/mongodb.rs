use mongodb::Client;

pub async fn create_mongodb_client(uri: String) -> mongodb::error::Result<Client> {
    let client = Client::with_uri_str(&uri).await?;
    Ok(client)
}
