use anyhow::Result;

pub trait CreateRound {
    fn create_round(&mut self) -> Result<()>;
}