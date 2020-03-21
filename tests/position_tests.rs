use pnlsim::Currency::*;
use pnlsim::Security::*;
use pnlsim::*;

#[test]
fn test_basic_tx() {
    let mut portfolio = Portfolio::new();
    assert!(portfolio.is_empty());

    let fv = Future(FuturesContract::new(String::from("FVU0"), 1000, USD));
    // buy 100FV@120 cumulative position should be 100
    tx(&mut portfolio, &fv, 100.0, 120.0);
    assert_eq!(portfolio.len(), 1);
    match portfolio.get(&fv) {
        Some(&h) => {
            assert_eq!(h.quantity, 100.0);
            assert_eq!(h.price, 120.0);
        }
        _ => assert!(false),
    }
    // buy 100FV@121, cumulative position should be 200, avg price 120.5
    tx(&mut portfolio, &fv, 100.0, 121.0);
    assert_eq!(portfolio.len(), 1);
    match portfolio.get(&fv) {
        Some(&h) => {
            assert_eq!(h.quantity, 200.0);
            assert_eq!(h.price, 120.5);
        }
        _ => assert!(false),
    }
    // sell 200FV@122, position should be removed
    // cash (122 - 120.5) * 1000 * 200
    tx(&mut portfolio, &fv, -200.0, 122.0);
    assert_eq!(portfolio.len(), 1);
    match portfolio.get(&fv) {
        Some(&_h) => assert!(false),
        _ => assert!(true),
    }
    match portfolio.get(&Cash(USD)) {
        Some(&h) => assert_eq!(h.quantity, 300e+03),
        _ => assert!(false),
    }

    // double buy @119.0
    tx(&mut portfolio, &fv, 300.0, 119.0);
    tx(&mut portfolio, &fv, 300.0, 119.0);
    match portfolio.get(&fv) {
        Some(&h) => {
            assert_eq!(h.quantity, 600.0);
            assert_eq!(h.price, 119.0);
        }
        _ => println!("Error FV not found."),
    }
}

#[test]
fn test_closeout_same_price() {
    let mut portfolio = Portfolio::new();
    assert!(portfolio.is_empty());
    let fv = Future(FuturesContract::new(String::from("FVU0"), 1000, USD));

    tx(&mut portfolio, &fv, 100.0, 100.0);
    assert_eq!(portfolio.len(), 1);
    match portfolio.get(&fv) {
        Some(&h) => {
            assert_eq!(h.quantity, 100.0);
            assert_eq!(h.price, 100.0);
        }
        _ => assert!(false),
    }
    tx(&mut portfolio, &fv, -100.0, 100.0);
    // closeout at same price should create no cash
    assert!(portfolio.is_empty());
}

#[test]
fn test_position_flip_cost_basis() {
    let mut portfolio = Portfolio::new();
    assert!(portfolio.is_empty());

    let fv = Future(FuturesContract::new(String::from("FVU0"), 1000, USD));

    tx(&mut portfolio, &fv, 100.0, 101.0);
    assert_eq!(portfolio.len(), 1);
    match portfolio.get(&fv) {
        Some(&h) => {
            assert_eq!(h.quantity, 100.0);
            assert_eq!(h.price, 101.0);
        }
        _ => assert!(false),
    }
    // flip position, cost basis should be new price
    tx(&mut portfolio, &fv, -500.0, 102.0);
    match portfolio.get(&fv) {
        Some(&h) => {
            assert_eq!(h.quantity, -400.0);
            assert_eq!(h.price, 102.0);
        }
        _ => assert!(false),
    }

    // cash created only on the original position (100 contracts)
    match portfolio.get(&Cash(USD)) {
        Some(&h) => assert_eq!(h.quantity, 100e3),
        _ => assert!(false),
    }
}
