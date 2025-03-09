//! 日期格式化
pub mod datetime_format {
    use chrono::NaiveDateTime as DateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::from(dt))
    }
}

pub mod datetime_option_format {
    use chrono::NaiveDateTime as DateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Some(d) = date else {
            return serializer.serialize_none();
        };
        let s = format!("{}", d.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        let Some(t) = s else {
            return Ok(None);
        };
        let dt = DateTime::parse_from_str(&t, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(Some(DateTime::from(dt)))
    }
}
