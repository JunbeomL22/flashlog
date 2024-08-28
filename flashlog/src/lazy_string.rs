use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub struct LazyString {
    cell: OnceCell<String>,
    initializer: Box<dyn Fn() -> String + Send + Sync>,
}

impl LazyString {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        LazyString {
            cell: OnceCell::new(),
            initializer: Box::new(f),
        }
    }
}

impl Deref for LazyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        self.cell.get_or_init(|| (self.initializer)())
    }
}

impl Serialize for LazyString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Force evaluation and serialize the result
        self.deref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LazyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(LazyString::new(move || s.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lazy_string() {
        let lazy_string = LazyString::new(|| format!("{} {} {}", 1, 2, 3));
        let json_string = serde_json::to_string(&lazy_string).unwrap();
        println!("{}", json_string);
        assert_eq!("1 2 3", *lazy_string);
    }
}