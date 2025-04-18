use application::repository::ITeamRepo;
use db::repository::TeamRepo;

#[test]
fn select_all_teams() {
    let mut repo = TeamRepo::new();

    let ids = repo.all_teams_id();

    assert_eq!(15, ids.len());
}