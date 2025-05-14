use serde::{self, Deserialize, Deserializer, Serializer};

pub mod string_as_float {
    use super::*;

    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<f64>().map_err(serde::de::Error::custom)
    }
}

pub mod option_string_as_float {
    use super::*;

    pub fn serialize<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        opt.map(|s| s.parse::<f64>().map_err(serde::de::Error::custom))
            .transpose()
    }
}

fn parse_datetime<E: serde::de::Error>(s: &str) -> Result<chrono::DateTime<chrono::Utc>, E> {
    match chrono::DateTime::parse_from_rfc3339(&s).map(|d| d.to_utc()) {
        Ok(d) => Ok(d),
        Err(err) => {
            if err.kind() == chrono::format::ParseErrorKind::TooShort {
                let d = chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|err| {
                    serde::de::Error::custom(format!("Failed to parse date: {}", err))
                })?;
                let result = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                    d.and_hms_opt(0, 0, 0).unwrap_or_default(),
                    chrono::Utc,
                );
                return Ok(result);
            }
            return Err(serde::de::Error::custom(format!(
                "Failed to parse date: {}",
                err
            )));
        }
    }
}
pub mod iso8601 {
    use super::*;

    pub fn serialize<S>(value: &crate::Time, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<crate::Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_datetime(&s)
    }
}

pub mod option_iso8601 {
    use super::*;

    pub fn serialize<S>(value: &Option<crate::Time>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_rfc3339()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<crate::Time>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        opt.map(|s| parse_datetime(&s)).transpose()
    }
}

#[cfg(test)]
mod tests {
    use crate::Time;

    use super::*;

    #[test]
    fn test_option_iso8601() {
        let json = r#"
        {
            "from_date": "2022-12-29",
            "from_date2": "2025-05-11T19:21:09Z"
        }
        "#;

        #[derive(Debug, Deserialize)]
        struct Test {
            #[serde(with = "option_iso8601")]
            from_date: Option<Time>,
            #[serde(with = "iso8601")]
            from_date2: Time,
        }
        let parsed: Test = serde_json::from_str(json).unwrap();
        println!("{:?}", parsed);
    }
}
