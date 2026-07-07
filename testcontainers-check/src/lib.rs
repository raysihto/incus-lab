#[cfg(test)]
mod tests {
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;

    #[tokio::test]
    async fn postgres_starts() {
        let container = Postgres::default().start().await.unwrap();
        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let host = container.get_host().await.unwrap();
        eprintln!("postgres listening on {host}:{port}");
    }

    #[tokio::test]
    async fn minio_starts() {
        use testcontainers_modules::minio::MinIO;

        let container = MinIO::default().start().await.unwrap();
        let port = container.get_host_port_ipv4(9000).await.unwrap();
        let host = container.get_host().await.unwrap();

        let url = format!("http://{host}:{port}/minio/health/ready");
        let resp = reqwest::get(&url).await.unwrap();
        eprintln!("minio health: {}", resp.status());
        assert!(resp.status().is_success());
    }
}
