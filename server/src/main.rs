fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use solana_client::nonblocking::rpc_client::RpcClient;

    #[test]
    fn first_test() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn can_connect_to_solana() {
        let rpc_url = "https://api.devnet.solana.com".to_string();
        let client = RpcClient::new(rpc_url);
        let version = client.get_version().await;
        assert!(
            version.is_ok(),
            "Failed to get Solana version: {:?}",
            version.err()
        );
    }
}
