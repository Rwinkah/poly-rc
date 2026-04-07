pub mod clob_client;
pub mod public;
pub mod shared;

#[cfg(test)]
mod tests {
    use crate::clob_client::models::ClobClientArgs;

    use super::*;

    #[tokio::test]
    async fn it_works() {}
}
