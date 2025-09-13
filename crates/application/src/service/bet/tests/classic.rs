use super::super::*;

#[test]
fn normalize_eq() {
    let mut probs = [0.1, 0.1, 0.1];

    BetCalculator::normalize(&mut probs);

    assert!((probs.iter().sum::<f64>() - 1.).abs() < EPS);
}

#[test]
fn normalize_single() {
    let mut probs = [0.1];

    BetCalculator::normalize(&mut probs);

    assert!((probs.iter().sum::<f64>() - 1.).abs() < EPS);
}

#[test]
fn normalize_diff() {
    let mut probs = [0.3, 120., 1.076123];

    BetCalculator::normalize(&mut probs);

    assert!((probs.iter().sum::<f64>() - 1.).abs() < EPS);
}

#[test]
fn calculate_total_coefficients() {
    let total = 2;
    let mut totals = PastTotals::new(total);
    totals.add_total(0);
    totals.add_total(1);
    totals.add_total(1);
    totals.add_total(1);
    totals.add_total(2);
    totals.add_total(3);
    let margin = Margin::try_from(0.12).unwrap();

    let res = BetCalculator::calculate_total_coefficients(total, totals, margin).unwrap();

    let mut sum = 0.;
    for (_, coefficient) in res {
        sum += 1. / f64::from(coefficient);
    }
    assert!(sum > 1.);
}

#[test]
fn calculate_winner_coefficients() {
    let home_res = PastResults {
        wins: 6,
        draws: 15,
        loses: 4,
    };
    let guest_res = PastResults {
        wins: 4,
        draws: 9,
        loses: 12,
    };
    let h2h_res = PastResults {
        wins: 5,
        draws: 8,
        loses: 12,
    };
    let margin = Margin::try_from(0.12).unwrap();

    let res =
        BetCalculator::calculate_winner_coefficients(home_res, guest_res, h2h_res, 60, 25, margin)
            .unwrap();

    let mut sum = 0.;
    for (_, coefficient) in res {
        sum += 1. / f64::from(coefficient);
    }
    assert!(sum > 1.);
}
