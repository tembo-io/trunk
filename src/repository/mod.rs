pub mod local;

use crate::package::Package;
use async_trait::async_trait;

#[async_trait]
pub trait WriteableRepository {
    type Output;
    type Error;

    async fn publish(&mut self, package: &mut Package) -> Result<Self::Output, Self::Error>;
}
