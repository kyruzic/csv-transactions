#[derive(Debug, Clone, PartialEq)]
pub struct FixedNumber {
    value: i64,
    decimals: usize,
}

impl FixedNumber {
    // A Fixed Number/Fixed Point number is a way to represent a floating point safely, by storing the number only as a integer
    // whenever math is done then we are doing integer math rather than floating point math which ensures safety. When we want to display
    // this value we convert it to a floating point with the specified number of decimals
    pub fn new() -> FixedNumber {
        FixedNumber {
            value: 0,
            decimals: 4,
        }
    }

    pub fn add(x: &FixedNumber, y: &FixedNumber) -> FixedNumber {
        FixedNumber {
            value: x.value + y.value,
            decimals: x.decimals,
        }
    }

    pub fn subtract(x: &FixedNumber, y: &FixedNumber) -> FixedNumber {
        FixedNumber {
            value: x.value - y.value,
            decimals: x.decimals,
        }
    }

    pub fn from_float(x: f64) -> FixedNumber {
        FixedNumber {
            value: (x * 10.0_f64.powi(4)).floor() as i64,
            decimals: 4,
        }
    }

    pub fn gt(&self, value: i64) -> bool {
        self.value > value
    }

    pub fn gt_eq(&self, num: &FixedNumber) -> bool {
        self.value > num.value
    }

    pub fn get_displayed_value(&self) -> String {
        let value = self.value as f64 * 10_f64.powi(-(self.decimals as i32));
        format!("{:.*}", self.decimals, value)
    }
}
