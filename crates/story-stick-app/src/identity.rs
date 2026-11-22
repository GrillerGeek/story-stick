//! Application identity and preference access.
#[cfg(not(target_arch = "wasm32"))]
pub const APP_ID: &str = "io.github.GrillerGeek.StoryStick";

pub fn preference_string(storage: &dyn eframe::Storage, key: &str) -> Option<String> {
    storage.get_string(key)
}

pub fn preference_value<T: serde::de::DeserializeOwned>(
    storage: &dyn eframe::Storage,
    key: &str,
) -> Option<T> {
    eframe::get_value(storage, key)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn data_override() -> Option<std::path::PathBuf> {
    std::env::var_os("STORY_STICK_DATA_DIR").map(std::path::PathBuf::from)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn preferences_path() -> Result<std::path::PathBuf, String> {
    if let Some(path) = data_override() {
        return Ok(path.join("preferences.ron"));
    }
    let new = eframe::storage_dir(APP_ID).ok_or("Cannot locate application preferences")?;
    Ok(new.join("app.ron"))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn legacy_preference_keys_are_not_restored() {
        #[derive(Default)]
        struct Storage(std::collections::BTreeMap<String, String>);
        impl eframe::Storage for Storage {
            fn get_string(&self, key: &str) -> Option<String> {
                self.0.get(key).cloned()
            }
            fn set_string(&mut self, key: &str, value: String) {
                self.0.insert(key.into(), value);
            }
            fn remove_string(&mut self, key: &str) {
                self.0.remove(key);
            }
            fn flush(&mut self) {}
        }
        let mut storage = Storage::default();
        eframe::set_value(&mut storage, "ketchup.inspector.v1", &42_u32);
        assert_eq!(preference_value::<u32>(&storage, "story-stick.inspector.v1"), None);
        assert_eq!(preference_string(&storage, "story-stick.inspector.v1"), None);
        eframe::set_value(&mut storage, "story-stick.inspector.v1", &7_u32);
        assert_eq!(preference_value::<u32>(&storage, "story-stick.inspector.v1"), Some(7));
    }
}
