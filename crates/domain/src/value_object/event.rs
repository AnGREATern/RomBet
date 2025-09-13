use std::cmp::Ordering;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Event {
    WDL(Winner),
    T(EventTotal),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::WDL(winner) => write!(f, "{}", winner),
            Event::T(et) => write!(f, "{}", et),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Winner {
    W1,
    X,
    W2,
}

impl fmt::Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Winner::W1 => write!(f, "П1"),
            Winner::X => write!(f, "X"),
            Winner::W2 => write!(f, "П2"),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EventTotal {
    pub total: u8,
    #[serde(with = "ordering_serde")]
    pub ordering: Ordering,
}

impl fmt::Display for EventTotal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ordering {
            Ordering::Greater => write!(f, "ТБ{}.5", self.total),
            Ordering::Equal => write!(f, "{} голов", self.total),
            Ordering::Less => write!(f, "ТМ{}.5", self.total - 1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct OrderingWrapper(Ordering);

impl Serialize for OrderingWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Ordering::Less => serializer.serialize_i8(-1),
            Ordering::Equal => serializer.serialize_i8(0),
            Ordering::Greater => serializer.serialize_i8(1),
        }
    }
}

impl<'de> Deserialize<'de> for OrderingWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: i8 = Deserialize::deserialize(deserializer)?;
        match value {
            -1 => Ok(OrderingWrapper(Ordering::Less)),
            0 => Ok(OrderingWrapper(Ordering::Equal)),
            1 => Ok(OrderingWrapper(Ordering::Greater)),
            _ => Err(serde::de::Error::custom(format!(
                "invalid value: {}",
                value
            ))),
        }
    }
}

mod ordering_serde {
    use super::*;

    pub fn serialize<S>(order: &Ordering, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        OrderingWrapper(*order).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Ordering, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wrapper = OrderingWrapper::deserialize(deserializer)?;
        Ok(wrapper.0)
    }
}
