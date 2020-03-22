use chrono::NaiveDate;
use std::collections::HashMap;
use std::f32::NAN;
use std::ops::{Add, Mul, Sub};
use Currency::*;

pub type Portfolio = HashMap<Security, Holding>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    USD,
    CAD,
    JPY,
    EUR,
    GBP,
    AUD,
    NZD,
    NOK,
    SEK,
}

#[derive(Debug, Clone, Copy)]
pub struct Holding {
    pub quantity: f32,
    pub price: f32,
    pub mark: f32,
}

impl Holding {
    pub fn new(quantity: f32, price: f32, mark: f32) -> Self {
        Self {
            quantity,
            price,
            mark,
        }
    }
    pub fn default() -> Self {
        Self {
            quantity: 0.0,
            price: NAN,
            mark: NAN,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Value {
    value: f32,
    currency: Currency,
}

impl Value {
    pub fn new(value: f32, currency: Currency) -> Self {
        Self { value, currency }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        if self.currency == other.currency {
            Value {
                value: self.value + other.value,
                currency: self.currency,
            }
        } else {
            Value {
                value: NAN,
                currency: self.currency,
            }
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        if self.currency == other.currency {
            Value {
                value: self.value - other.value,
                currency: self.currency,
            }
        } else {
            Value {
                value: NAN,
                currency: self.currency,
            }
        }
    }
}
impl Mul<f32> for Value {
    type Output = Value;

    fn mul(self, other: f32) -> Value {
        Value {
            value: self.value * other,
            currency: self.currency,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Security {
    Cash(Currency),                       // EUR
    Equity(String, Currency),             // IBM USD
    Future(String, i32, Currency),        // FVU0 1000 USD
    FxFrd(Currency, Currency, NaiveDate), // USD JPY 2020-04-01
}

impl Security {
    // price or marks map (for cross ccy for instance)
    fn value(&self, price: f32) -> Value {
        match self {
            Security::Cash(ccy) => Value {
                value: 1.0,
                currency: *ccy,
            },
            Security::Equity(_, ccy) => Value {
                value: price,
                currency: *ccy,
            },
            Security::Future(_, pv, ccy) => Value {
                value: *pv as f32 * price,
                currency: *ccy,
            },
            Security::FxFrd(_, settle_ccy, _) => Value {
                value: price,
                currency: *settle_ccy,
            },
        }
    }
    fn cash_settled(&self) -> bool {
        match self {
            Security::Cash(_) => true,
            Security::Equity(_, _) => true,
            Security::Future(_, _, _) => false,
            Security::FxFrd(_, _, _) => false,
        }
    }
}

pub fn wgt_avg(q1: f32, p1: f32, q2: f32, p2: f32) -> f32 {
    assert!(q1.signum() == q2.signum());
    let p1_wgt = q1 / (q1 + q2);
    let p2_wgt = 1.0 - p1_wgt;
    p1 * p1_wgt + p2 * p2_wgt
}

pub fn tx(
    portfolio: &mut HashMap<Security, Holding>,
    s: &Security,
    trade_quantity: f32,
    trade_price: f32,
) {
    let cash: Option<Value>;

    if trade_quantity == 0.0 {
        // invalid tx
        cash = None;
    } else {
        // find or create entry
        let h = portfolio.entry(s.clone()).or_insert(Holding::default());

        if h.quantity == 0.0 {
            // new position, new price
            h.price = trade_price;
            if s.cash_settled() {
                cash = Some(s.value(trade_price) * -trade_quantity);
            } else {
                cash = None;
            }
        } else if trade_quantity.signum() == h.quantity.signum() {
            // increment_position, wgt avg price
            h.price = wgt_avg(h.quantity, h.price, trade_quantity, trade_price);
            if s.cash_settled() {
                cash = Some(s.value(trade_price) * -trade_quantity);
            } else {
                cash = None;
            }
        } else if trade_quantity.abs() > h.quantity.abs() {
            // flip_position
            // exit cash is only on original position (not full trade size)
            if s.cash_settled() {
                cash = Some(s.value(trade_price) * -trade_quantity);
            } else {
                cash = Some((s.value(trade_price) - s.value(h.price)) * h.quantity);
            }
            h.price = trade_price; // new lot, replace price
        } else {
            // reduce_position
            // no change to price on position reduction
            if s.cash_settled() {
                cash = Some(s.value(trade_price) * -trade_quantity);
            } else {
                cash = Some((s.value(trade_price) - s.value(h.price)) * -trade_quantity);
            }
        }
        // update position
        h.quantity += trade_quantity;
        if h.quantity == 0.0 {
            portfolio.remove(&s);
        }
    };

    match cash {
        Some(cf) if cf.value != 0.0 => {
            let cash_holding = portfolio
                .entry(Security::Cash(cf.currency))
                .or_insert(Holding::default());
            cash_holding.quantity += cf.value;
            if cash_holding.quantity == 0.0 {
                portfolio.remove(&Security::Cash(cf.currency));
            }
        }
        _ => {}
    }
}
