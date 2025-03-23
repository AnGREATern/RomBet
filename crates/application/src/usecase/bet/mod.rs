mod make_bet;
mod make_report;

pub use make_bet::MakeBet;
pub use make_report::MakeReport;

pub trait IBetService: MakeBet + MakeReport { }