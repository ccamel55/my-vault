// https://github.com/serde-rs/serde/issues/1110#issuecomment-348822979

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::forward_to_deserialize_any;

/// Get name of all struct fields
pub fn struct_fields<'de, T>() -> &'static [&'static str]
where
    T: Deserialize<'de>,
{
    struct StructFieldsDeserializer<'a> {
        fields: &'a mut Option<&'static [&'static str]>,
    }

    impl<'de, 'a> Deserializer<'de> for StructFieldsDeserializer<'a> {
        type Error = de::value::Error;

        fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            Err(de::Error::custom("I'm just here for the fields"))
        }

        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            *self.fields = Some(fields);
            self.deserialize_any(visitor)
        }

        forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
            byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map enum identifier ignored_any
        }
    }

    let mut fields = None;
    let _ = T::deserialize(StructFieldsDeserializer {
        fields: &mut fields,
    });
    fields.unwrap()
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[tokio::test]
    async fn struct_fields() {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct Hello {
            my: bool,
            name: i32,
            is: String,
            jeff: &'static [u8],
        }

        let field_names = super::struct_fields::<Hello>();

        assert_eq!(field_names.len(), 4);

        assert_eq!(field_names[0], "my");
        assert_eq!(field_names[1], "name");
        assert_eq!(field_names[2], "is");
        assert_eq!(field_names[3], "jeff");
    }
}
