// https://github.com/serde-rs/serde/issues/1110#issuecomment-348823070

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::forward_to_deserialize_any;

/// Get name of all enum variants
pub fn enum_variants<'de, T>() -> &'static [&'static str]
where
    T: Deserialize<'de>,
{
    struct EnumVariantsDeserializer<'a> {
        variants: &'a mut Option<&'static [&'static str]>,
    }

    impl<'de, 'a> Deserializer<'de> for EnumVariantsDeserializer<'a> {
        type Error = de::value::Error;

        fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            Err(de::Error::custom("I'm just here for the variants"))
        }

        fn deserialize_enum<V>(
            self,
            _name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            *self.variants = Some(variants);
            self.deserialize_any(visitor)
        }

        forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
            byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map struct identifier ignored_any
        }
    }

    let mut variants = None;
    let _ = T::deserialize(EnumVariantsDeserializer {
        variants: &mut variants,
    });
    variants.unwrap()
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[tokio::test]
    async fn enum_variants() {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        enum Bye {
            The,
            Earth,
            Is,
            Flat,
        }

        let enum_variants = super::enum_variants::<Bye>();

        assert_eq!(enum_variants.len(), 4);

        assert_eq!(enum_variants[0], "The");
        assert_eq!(enum_variants[1], "Earth");
        assert_eq!(enum_variants[2], "Is");
        assert_eq!(enum_variants[3], "Flat");
    }
}
