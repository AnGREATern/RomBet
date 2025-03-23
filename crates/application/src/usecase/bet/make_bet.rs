use anyhow::Result;

pub trait MakeBet {
    fn make_bet(&self) -> Result<()>;
}