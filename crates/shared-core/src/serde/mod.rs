mod enum_variants;
mod struct_fields;

pub use enum_variants::*;
pub use struct_fields::*;

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
