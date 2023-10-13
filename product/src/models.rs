use serde::{Deserialize, Serialize};
use time::Date;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub image_url: String,
    #[serde(with = "date_format")]
    pub created_at: Date,
    pub tags: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
pub struct NewProduct {
    pub name: ProductName,
    pub price: f32,
    pub description: ProductDescription,
    #[serde(default)]
    pub image_url: Url,
    pub tags: Option<Vec<ProductTag>>,
}

macro_rules! new_string_type {
    ($type:ident, max_length = $max_length:expr, error = $error_message:expr) => {
        #[derive(Clone, Serialize, Deserialize)]
        #[serde(try_from = "String")]
        #[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
        #[cfg_attr(feature = "sqlx", sqlx(transparent))]
        pub struct $type(String);

        impl $type {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $type {
            type Error = &'static str;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if value.len() <= $max_length {
                    Ok($type(value))
                } else {
                    Err($error_message)
                }
            }
        }

        impl From<$type> for String {
            fn from(value: $type) -> Self {
                value.0
            }
        }

        impl Default for $type {
            fn default() -> Self {
                $type(String::new())
            }
        }
    };
}

new_string_type!(
    ProductName,
    max_length = 100,
    error = "product name is too long, max 100 characters"
);

new_string_type!(
    ProductDescription,
    max_length = 500,
    error = "product description is too long, max 500 characters"
);

new_string_type!(
    ProductTag,
    max_length = 50,
    error = "product tag is too long, max 50 characters"
);

new_string_type!(
    Url,
    max_length = 1000,
    error = "url is too long, max 1000 characters"
);

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
