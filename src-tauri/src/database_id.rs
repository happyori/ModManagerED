use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;
use surrealdb::sql::{Thing};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, MapAccess, Visitor};
use surrealdb::opt::Resource;
use specta::{DataType, Generics, Type, TypeMap};

#[derive(Debug, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DbID(
    #[serde(deserialize_with = "string_or_struct")]
    pub(crate) Thing
);

impl Type for DbID {
    fn inline(type_map: &mut TypeMap, generics: Generics) -> DataType {
        <String as Type>::inline(type_map, generics)
    }
}

impl Deref for DbID {
    type Target = Thing;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Thing> for DbID {
    fn as_ref(&self) -> &Thing {
        &self.0
    }
}

impl From<DbID> for Resource {
    fn from(value: DbID) -> Self {
        value.0.into()
    }
}

impl From<&DbID> for Resource {
    fn from(value: &DbID) -> Self {
        value.0.clone().into()
    }
}

impl Serialize for DbID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.0.to_raw())
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de> + FromStr,
        D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);
    
    impl<'de, T> Visitor<'de> for StringOrStruct<T> where T: Deserialize<'de> + FromStr {
        type Value = T;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            FromStr::from_str(v).map_err(|_| Error::custom("Failed to deserialize string"))
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }
    deserializer.deserialize_any(StringOrStruct(PhantomData))
}