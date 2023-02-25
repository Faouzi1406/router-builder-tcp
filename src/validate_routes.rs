use crate::{
    builder_traits::builder::BuildRoute,
    route_builder::{HttpResponse, Routes},
};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ValidationErrors {
    /// A character that isn't allow could in this case could for example be ? or / or \ etc
    NotAllowedCharacter(char),
}

pub trait Validate<T> {
    /// All the chars in the char vector will be checked
    /// If any of the chars are contained in the
    fn validate_chars(validate: &mut T, chars: Vec<char>) -> Result<&mut T, ValidationErrors>
    where
        T: BuildRoute<T>,
        T: HttpResponse,
    {
        let string_t = validate.build_string();
        println!("{string_t}");

        for char in chars {
            let contains = string_t.contains(char);
            if contains {
                return Err(ValidationErrors::NotAllowedCharacter(char));
            }
        }

        Ok(validate)
    }
}

impl<T> Validate<T> for Routes<T>
where
    T: HttpResponse,
    T: BuildRoute<T> + Clone + Default + PartialEq + std::fmt::Debug,
{
}

impl Validate<Routes<String>> for Routes<String> {}
