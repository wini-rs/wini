use {
    super::{config::TomlLoadingError, err::ExitWithMessageIfErr},
    crate::{concat_paths, shared::wini::config::SERVER_CONFIG},
    serde::{Deserialize, Deserializer, de::Visitor},
    std::{collections::HashMap, io, sync::LazyLock},
};

#[derive(Debug)]
pub enum VecOrString {
    Vec(Vec<String>),
    String(String),
}

struct VecOrStringVisitor;

impl<'de> Visitor<'de> for VecOrStringVisitor {
    type Value = VecOrString;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or a vector of strings")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(value) = seq.next_element::<String>()? {
            vec.push(value);
        }
        Ok(VecOrString::Vec(vec))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(VecOrString::String(value.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(VecOrString::String(value))
    }
}


impl<'de> Deserialize<'de> for VecOrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(VecOrStringVisitor)
    }
}

/// The files on which a package depend on
pub static PACKAGES_FILES: LazyLock<HashMap<String, VecOrString>> = LazyLock::new(|| {
    fn module_path_from_short_name(package: &str, file: &str) -> String {
        if file.starts_with("https://") {
            file.to_string()
        } else {
            concat_paths!(
                &SERVER_CONFIG.path.modules,
                &package,
                std::path::Path::new(&file).file_name().unwrap_or_default()
            )
            .display()
            .to_string()
            .trim_start_matches('.')
            .to_string()
        }
    }

    let file_to_read_from = "./packages-files.toml";

    let file = std::fs::read_to_string(file_to_read_from)
        .map_err(|err| {
            match err.kind() {
                io::ErrorKind::NotFound => {
                    TomlLoadingError::ConfigFileDoesntExists(file_to_read_from.to_owned())
                },
                _ => TomlLoadingError::OtherIo(err),
            }
        })
        .exit_with_msg_if_err("Error while reading file");

    let hashmap: HashMap<String, VecOrString> =
        toml::from_str(&file).exit_with_msg_if_err("Unexpected error while parsing TOML");

    hashmap
        .into_iter()
        .map(|(key, vec_or_string)| {
            (
                key.clone(),
                match vec_or_string {
                    VecOrString::Vec(v) => {
                        VecOrString::Vec(
                            v.into_iter()
                                .map(|file| module_path_from_short_name(&key, &file))
                                .collect(),
                        )
                    },
                    VecOrString::String(s) => {
                        VecOrString::String(module_path_from_short_name(&key, &s))
                    },
                },
            )
        })
        .collect()
});
