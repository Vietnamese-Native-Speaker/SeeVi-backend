use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct JsonValueGetter {
    current_data: serde_json::Value,
}

impl<'a> std::ops::Deref for JsonValueGetter {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        &self.current_data
    }
}

impl JsonValueGetter {
    pub fn new(data: serde_json::Value) -> Self {
        Self { current_data: data }
    }

    pub fn field(mut self, key: &str) -> Self {
        self.current_data = self
            .current_data
            .get(key)
            .expect(&format!("should have '{}' field", key))
            .clone();
        self
    }

    pub fn bool(self, key: &str) -> bool {
        self.get(key)
            .expect(&format!("should have '{}' field", key))
            .as_bool()
            .unwrap()
    }

    pub fn get_unchecked(self, key: &str) -> serde_json::Value {
        self.current_data
            .get(key)
            .expect(&format!("should have '{}' field", key))
            .clone()
    }

    pub fn string(self, key: &str) -> String {
        self.get_unchecked(key).as_str().unwrap().to_owned()
    }

    pub fn i64(self, key: &str) -> i64 {
        self.get_unchecked(key).as_i64().unwrap()
    }

    pub fn u64(self, key: &str) -> u64 {
        self.get_unchecked(key).as_u64().unwrap()
    }

    pub fn f64(self, key: &str) -> f64 {
        self.get_unchecked(key).as_f64().unwrap()
    }

    pub fn array(self, key: &str) -> Vec<serde_json::Value> {
        self.get_unchecked(key).as_array().unwrap().to_owned()
    }

    pub fn object(self, key: &str) -> serde_json::Map<String, serde_json::Value> {
        self.get_unchecked(key).as_object().unwrap().to_owned()
    }
}
