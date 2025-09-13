use application::repository::ITeamRepo;
use db::init_pool;
use db::repository::TeamRepo;

#[test]
fn select_all_teams() {
    let pool = init_pool();

    let repo = TeamRepo::new(pool.clone());

    let ids = repo.all_teams_id();

    assert_eq!(15, ids.len());
}
