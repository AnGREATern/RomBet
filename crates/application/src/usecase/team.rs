use anyhow::Result;

pub trait CreateTeam {
    fn add_team<T: ToString>(&self, name: T) -> Result<()>;
}
