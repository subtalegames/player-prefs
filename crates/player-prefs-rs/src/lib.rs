use std::collections::HashMap;

pub enum PreferenceType {
    Bool(bool),
    Int(isize),
    Float(f64),
    String(String),
}

pub struct PlayerPrefs {
    preferences: HashMap<String, PreferenceType>,
}

impl PlayerPrefs {
    pub fn delete_all(&mut self) {
        self.preferences.clear();
    }

    pub fn delete_key(&mut self, key: &String) {
        self.preferences.remove(key);
    }

    #[cfg(feature = "bool")]
    pub fn get_bool(&self, key: &String) -> Option<&bool> {
        match self.preferences.get(key) {
            Some(PreferenceType::Bool(val)) => Some(val),
            _ => None,
        }
    }

    pub fn get_float(&self, key: &String) -> Option<&f64> {
        match self.preferences.get(key) {
            Some(PreferenceType::Float(val)) => Some(val),
            _ => None,
        }
    }

    pub fn get_int(&self, key: &String) -> Option<&isize> {
        match self.preferences.get(key) {
            Some(PreferenceType::Int(val)) => Some(val),
            _ => None,
        }
    }

    pub fn get_string(&self, key: &String) -> Option<&String> {
        match self.preferences.get(key) {
            Some(PreferenceType::String(val)) => Some(val),
            _ => None,
        }
    }

    pub fn has_key(&self, key: &String) -> bool {
        self.preferences.contains_key(key)
    }

    #[cfg(feature = "bool")]
    pub fn set_bool(&mut self, key: String, value: bool) -> Option<bool> {
        self.preferences
            .insert(key, PreferenceType::Bool(value))
            .and_then(|x| match x {
                PreferenceType::Bool(x) => Some(x),
                _ => None,
            })
    }

    pub fn set_float(&mut self, key: String, value: f64) -> Option<f64> {
        self.preferences
            .insert(key, PreferenceType::Float(value))
            .and_then(|x| match x {
                PreferenceType::Float(x) => Some(x),
                _ => None,
            })
    }

    pub fn set_int(&mut self, key: String, value: isize) -> Option<isize> {
        self.preferences
            .insert(key, PreferenceType::Int(value))
            .and_then(|x| match x {
                PreferenceType::Int(x) => Some(x),
                _ => None,
            })
    }

    pub fn set_string(&mut self, key: String, value: String) -> Option<String> {
        self.preferences
            .insert(key, PreferenceType::String(value))
            .and_then(|x| match x {
                PreferenceType::String(x) => Some(x),
                _ => None,
            })
    }
}
