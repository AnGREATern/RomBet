use anyhow::Result;

pub trait MakeReport {
    fn make_report(&self) -> Result<()>;
}