pub const SERVICE_ID: bluer::Uuid = bluer::Uuid::from_u128(488837762788578050050668711589115);

#[derive(Clone, Debug)]
pub struct Atc(bluer::Device);

impl Atc {
    pub async fn try_from_adapter(
        adapter: &bluer::Adapter,
        addr: bluer::Address,
    ) -> Result<Self, bluer::Error> {
        let device = adapter.device(addr)?;
        Ok(Atc(device))
    }
}
