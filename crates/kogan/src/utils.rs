use std::fmt;

use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Deserializer, de};

pub fn deserialize_date<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DateStringVisitor;

    impl<'de> de::Visitor<'de> for DateStringVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a datetime string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // 2021-10-26T13:35:49.115625
            NaiveDateTime::parse_from_str(v, "%FT%H:%M:%S%.6f")
                .map_err(E::custom)
                .map(|dt| DateTime::from_utc(dt, Utc))
        }
    }

    d.deserialize_any(DateStringVisitor)
}

#[test]
fn test_deserialize_date() {
    use serde::Deserialize;
    use chrono::TimeZone;
    #[derive(Deserialize)]
    struct Container {
        #[serde(deserialize_with="deserialize_date")]
        date: DateTime<Utc>,
    }
    let v = serde_json::from_str::<Container>(r#"{"date": "2021-10-26T13:35:49.115625"}"#).unwrap().date;
    let expected = Utc.ymd(2021, 10, 26).and_hms_micro(13, 35, 49, 115625);
    assert_eq!(v, expected);
}