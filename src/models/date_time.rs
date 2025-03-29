use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime, Time};

#[derive(Debug, Clone, PartialEq)]
pub struct DateTime(PrimitiveDateTime);

impl DateTime {
    pub fn date(&self) -> Date {
        self.0.date()
    }
    pub fn time(&self) -> Time {
        self.0.time()
    }
    pub fn datetime(&self) -> PrimitiveDateTime {
        self.0
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let format = time::format_description::well_known::Iso8601::DEFAULT;
        self.0
            .format(&format)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let format = time::format_description::well_known::Iso8601::DEFAULT;
        PrimitiveDateTime::parse(&s, &format)
            .map(DateTime)
            .map_err(serde::de::Error::custom)
    }
}
