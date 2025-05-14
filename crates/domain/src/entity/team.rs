use crate::value_object::Id;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Team {
    id: Id<Team>,
    name: String,
}

impl Team {
    pub fn new(id: Id<Self>, name: String) -> Self {
        Team {id, name}
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> Id<Team> {
        self.id
    }
}