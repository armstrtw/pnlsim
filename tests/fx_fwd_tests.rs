use chrono::NaiveDate;
use pnlsim::Currency::*;
use pnlsim::Security::*;
use pnlsim::*;

#[test]
fn fxfrd_basic_tx() {
    let mut portfolio = Portfolio::new();
    assert!(portfolio.is_empty());

    let usdjpy = FxFrd(USD, JPY, NaiveDate::from_ymd(2020, 3, 31));

    tx(&mut portfolio, &usdjpy, 1e6, 107.0);
    assert_eq!(portfolio.len(), 1);
    match portfolio.get(&usdjpy) {
        Some(&h) => {
            assert_eq!(h.quantity, 1e6);
            assert_eq!(h.price, 107.0);
        }
        _ => assert!(false),
    }

    tx(&mut portfolio, &usdjpy, 1e6, 108.0);
    assert_eq!(portfolio.len(), 1);
    match portfolio.get(&usdjpy) {
        Some(&h) => {
            assert_eq!(h.quantity, 2e6);
            assert_eq!(h.price, 107.5);
        }
        _ => assert!(false),
    }
    tx(&mut portfolio, &usdjpy, -2e6, 108.0);
    println!("{:?}", portfolio);
}
