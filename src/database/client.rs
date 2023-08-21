use aws_sdk_dynamodb::Client;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn get_client() -> &'static Client {
    CLIENT
        .get_or_init(|| async {
            let config = aws_config::load_from_env().await;
            Client::new(&config)
        })
        .await
}
