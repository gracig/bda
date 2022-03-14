pub mod datastore {
    use bdaproto::bda_client::BdaClient;
    use std::error::Error;
    use tonic::{transport::Channel, Request};

    pub async fn print_kinds(client: &mut BdaClient<Channel>) -> Result<(), Box<dyn Error>> {
        println!(
            "kinds {:?}",
            client
                .get_kinds(Request::new(bdaproto::GetKindsRequest {}))
                .await?
        );
        Ok(())
    }
}
