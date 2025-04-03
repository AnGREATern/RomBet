use domain::{entity::Team, value_object::Id};

pub trait ITeamRepo {
    fn new() -> Self;

    fn all_teams_id(&self) -> Vec<Id<Team>>;
}
