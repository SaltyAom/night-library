use bigdecimal::BigDecimal;
use juniper::graphql_scalar;

use std::str::FromStr;

use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Numeric;

use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromSqlRow, AsExpression)]
#[sql_type = "Numeric"]
pub struct MyBigDecimal(pub BigDecimal);

impl Serialize for MyBigDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("MyBigDecimal", &self.0)
    }
}

impl MyBigDecimal {
    pub fn from_str(value: &str) -> MyBigDecimal {
        MyBigDecimal(BigDecimal::from_str(value).unwrap())
    }
}

struct MyBigDecimalVisitor;

impl<'de> Visitor<'de> for MyBigDecimalVisitor {
    type Value = MyBigDecimal;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Decimal represented as a string.")
    }

    fn visit_str<E>(self, value: &str) -> Result<MyBigDecimal, E>
    where
        E: de::Error,
    {
        Ok(MyBigDecimal(BigDecimal::from_str(&value).unwrap()))
    }

    fn visit_f32<E>(self, value: f32) -> Result<MyBigDecimal, E>
    where
        E: de::Error,
    {
        Ok(MyBigDecimal(BigDecimal::from(value)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<MyBigDecimal, E>
    where
        E: de::Error,
    {
        Ok(MyBigDecimal(BigDecimal::from(value)))
    }
}

impl<'de> Deserialize<'de> for MyBigDecimal {
    fn deserialize<D>(deserializer: D) -> Result<MyBigDecimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(MyBigDecimalVisitor)
    }
}

impl<DB: Backend> ToSql<Numeric, DB> for MyBigDecimal
where
    BigDecimal: ToSql<Numeric, DB>,
{
    fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, DB>) -> ::diesel::serialize::Result {
        ToSql::<Numeric, DB>::to_sql(&self.0, out)
    }
}

impl<DB: Backend> FromSql<Numeric, DB> for MyBigDecimal
where
    BigDecimal: FromSql<Numeric, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        FromSql::<Numeric, DB>::from_sql(bytes).map(MyBigDecimal)
    }
}

use juniper::{ParseScalarResult, ParseScalarValue, Value};

graphql_scalar!(MyBigDecimal where Scalar = <S> {
    description: "BigDecimal"

    resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    from_input_value(v: &InputValue) -> Option<MyBigDecimal> {
        v.as_scalar_value::<String>()
        .and_then(|str_val|
            match BigDecimal::from_str(&str_val) {
            Ok(big_decimal) => Some(MyBigDecimal(big_decimal)),
            Err(_) => None,
        })
    }

    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
});
