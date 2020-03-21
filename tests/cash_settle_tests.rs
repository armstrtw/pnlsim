use pnlsim::Currency::*;
use pnlsim::Security::*;
use pnlsim::*;

// 5 basic tests
// new
// increment
// decrease
// closeout
// flip

#[test]
fn test_eq_tx() {
    let mut portfolio = Portfolio::new();
    assert!(portfolio.is_empty());

    let spy = Equity(String::from("SPY"), USD);
    tx(&mut portfolio, &spy, 1e3, 230.0);
    // new position test
    // creates a holding and a cash position
    assert_eq!(portfolio.len(), 2);
    match portfolio.get(&spy) {
        Some(&h) => {
            assert_eq!(h.quantity, 1e3);
            assert_eq!(h.price, 230.0);
        }
        _ => assert!(false),
    }

    match portfolio.get(&Cash(USD)) {
        Some(&h) => assert_eq!(h.quantity, -1e3 * 230.0),
        _ => assert!(false),
    }

    // buy same price
    tx(&mut portfolio, &spy, 1e3, 230.0);
    assert_eq!(portfolio.len(), 2);
    match portfolio.get(&spy) {
        Some(&h) => {
            assert_eq!(h.quantity, 2e3);
            assert_eq!(h.price, 230.0);
        }
        _ => assert!(false),
    }

    match portfolio.get(&Cash(USD)) {
        Some(&h) => assert_eq!(h.quantity, -2e3 * 230.0),
        _ => assert!(false),
    }
    // close position, same price
    // 0 balances in spy and cash are cleared
    tx(&mut portfolio, &spy, -2e3, 230.0);
    assert!(portfolio.is_empty());
}

#[test]
fn test_eq_increase_holdings() {
    let mut portfolio = Portfolio::new();
    assert!(portfolio.is_empty());

    let spy = Equity(String::from("SPY"), USD);
    tx(&mut portfolio, &spy, 1e3, 230.0);
    tx(&mut portfolio, &spy, 1e3, 231.0);
    assert_eq!(portfolio.len(), 2);
    match portfolio.get(&spy) {
        Some(&h) => {
            assert_eq!(h.quantity, 2e3);
            assert_eq!(h.price, 230.5); // price is wgt avg
        }
        _ => assert!(false),
    }

    match portfolio.get(&Cash(USD)) {
        Some(&h) => assert_eq!(h.quantity, -1e3 * 230.0 + -1e3 * 231.0),
        _ => assert!(false),
    }
}

#[test]
fn test_eq_decrease_holdings() {
    let mut portfolio = Portfolio::new();
    assert!(portfolio.is_empty());

    let spy = Equity(String::from("SPY"), USD);
    tx(&mut portfolio, &spy, 1e3, 230.0);
    assert_eq!(portfolio.len(), 2);
    match portfolio.get(&spy) {
        Some(&h) => {
            assert_eq!(h.quantity, 1e3);
            assert_eq!(h.price, 230.0); // price is wgt avg
        }
        _ => assert!(false),
    }

    match portfolio.get(&Cash(USD)) {
        Some(&h) => assert_eq!(h.quantity, -1e3 * 230.0),
        _ => assert!(false),
    }

    tx(&mut portfolio, &spy, -500.0, 232.0);
    assert_eq!(portfolio.len(), 2);
    match portfolio.get(&spy) {
        Some(&h) => {
            assert_eq!(h.quantity, 500.0);
            assert_eq!(h.price, 230.0); // price doesn't change
        }
        _ => assert!(false),
    }

    match portfolio.get(&Cash(USD)) {
        Some(&h) => assert_eq!(h.quantity, -1e3 * 230.0 + 500.0 * 232.0),
        _ => assert!(false),
    }
}

#[test]
fn test_basic_eq_flip() {
    let mut portfolio = Portfolio::new();
    assert!(portfolio.is_empty());

    let spy = Equity(String::from("SPY"), USD);
    tx(&mut portfolio, &spy, 1e3, 230.0);
    assert_eq!(portfolio.len(), 2);
    match portfolio.get(&spy) {
        Some(&h) => {
            assert_eq!(h.quantity, 1e3);
            assert_eq!(h.price, 230.0);
        }
        _ => assert!(false),
    }

    match portfolio.get(&Cash(USD)) {
        Some(&h) => assert_eq!(h.quantity, -1e3 * 230.0),
        _ => assert!(false),
    }

    tx(&mut portfolio, &spy, -2e3, 231.0);

    match portfolio.get(&spy) {
        Some(&h) => {
            assert_eq!(h.quantity, -1e3);
            assert_eq!(h.price, 231.0);
        }
        _ => assert!(false),
    }

    match portfolio.get(&Cash(USD)) {
        Some(&h) => assert_eq!(h.quantity, 232e3),
        _ => assert!(false),
    }
}
