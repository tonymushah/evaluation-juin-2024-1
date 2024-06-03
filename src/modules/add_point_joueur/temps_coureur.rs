use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct TempsCoureur(pub i32);

impl TempsCoureur {
    pub fn parse<I: Into<String>>(i: I) -> crate::Result<Self> {
        let i = i.into();
        let reg = Regex::new(r"(?<heures>[\d+]*):(?<minutes>[\d]*):(?<secondes>[\d]*)")?;
        let Some(captures) = reg.captures(&i) else {
            return Ok(Self(i.parse::<i32>()?));
        };
        let heures = captures
            .name("heures")
            .ok_or(crate::Error::RegexCaptureNameNotFound(String::from(
                "heures",
            )))?
            .as_str()
            .parse::<i32>()?;
        let minutes = captures
            .name("minutes")
            .ok_or(crate::Error::RegexCaptureNameNotFound(String::from(
                "minutes",
            )))?
            .as_str()
            .parse::<i32>()?;
        let secondes = captures
            .name("secondes")
            .ok_or(crate::Error::RegexCaptureNameNotFound(String::from(
                "secondes",
            )))?
            .as_str()
            .parse::<i32>()?;
        Ok(Self(heures * 3600 + minutes * 60 + secondes))
    }
}

#[Scalar]
impl ScalarType for TempsCoureur {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => Ok(Self::parse(s)?),
            Value::Number(i) => Ok(Self(
                i.as_u64().ok_or(crate::Error::TryFromInt)?.try_into()?,
            )),
            _ => Err(InputValueError::expected_type(value)),
        }
    }
    fn to_value(&self) -> Value {
        Value::Number(self.0.into())
    }
}
