use anyhow::{Result, bail};

use std::{cmp::Ordering, ops::Add};

pub struct PastTotals {
    origin_total: u8,
    less: u8,
    equal: u8,
    greater: u8,
}

impl PastTotals {
    pub fn new(origin_total: u8) -> Self {
        let less = 0;
        let equal = 0;
        let greater = 0;
        Self {
            origin_total,
            less,
            equal,
            greater,
        }
    }

    pub fn add_total(&mut self, total: u8) {
        match self.origin_total.cmp(&total) {
            Ordering::Less => self.less += 1,
            Ordering::Equal => self.equal += 1,
            Ordering::Greater => self.greater += 1,
        }
    }

    pub fn total(&self) -> u8 {
        self.origin_total
    }

    pub fn less(&self) -> u8 {
        self.less
    }

    pub fn equal(&self) -> u8 {
        self.equal
    }

    pub fn greater(&self) -> u8 {
        self.greater
    }

    pub fn size(&self) -> u8 {
        self.less + self.equal + self.greater
    }
}

impl Add for PastTotals {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.origin_total != rhs.origin_total {
            bail!("Can't add PastTotals with different origin_total")
        }
        let origin_total = self.origin_total;
        let less = self.less + rhs.less;
        let equal = self.equal + rhs.equal;
        let greater = self.greater + rhs.greater;
        Ok(Self {
            origin_total,
            less,
            equal,
            greater,
        })
    }
}
