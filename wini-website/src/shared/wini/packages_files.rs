use {
    super::config::TomlLoadingError,
    crate::{concat_paths, shared::wini::config::SERVER_CONFIG},
    serde::{de::Visitor, Deserialize, Deserializer},
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

pub static PACKAGES_FILES: LazyLock<HashMap<String, VecOrString>> = LazyLock::new(|| {
    let file_to_read_from = "./packages-files.toml";

    let file = match std::fs::read_to_string(file_to_read_from) {
        Ok(s) => s,
        Err(err) => {
            let err_kind = match err.kind() {
                io::ErrorKind::NotFound => {
                    TomlLoadingError::ConfigFileDoesntExists(file_to_read_from.to_owned())
                },
                _ => TomlLoadingError::OtherIo(err),
            };
            log::error!("{err_kind:#?}");
            std::process::exit(1);
        },
    };

    let hashmap: HashMap<String, VecOrString> = match toml::from_str(&file) {
        Ok(hm) => hm,
        Err(err) => {
            log::error!("{err:#?}");
            std::process::exit(1);
        },
    };


    hashmap
        .into_iter()
        .map(|(key, vec_or_string)| {
            (
                key.clone(),
                match vec_or_string {
                    VecOrString::Vec(v) => {
                        VecOrString::Vec(
                            v.into_iter()
                                .map(|file| {
                                    concat_paths!(
                                        &SERVER_CONFIG.path.modules,
                                        &key,
                                        std::path::Path::new(&file).file_name().unwrap()
                                    )
                                    .display()
                                    .to_string()
                                })
                                .collect(),
                        )
                    },
                    VecOrString::String(s) => {
                        VecOrString::String(
                            concat_paths!(
                                &SERVER_CONFIG.path.modules,
                                &key,
                                std::path::Path::new(&s).file_name().unwrap()
                            )
                            .display()
                            .to_string(),
                        )
                    },
                },
            )
        })
        .collect()
});

pub trait CssFilesFromPackage {
    fn css_files_from_package(&self, pkg: &str) -> Option<Vec<String>>;
}

impl CssFilesFromPackage for HashMap<String, VecOrString> {
    fn css_files_from_package(&self, pkg: &str) -> Option<Vec<String>> {
        self.get(pkg).and_then(|files| {
            match files {
                VecOrString::String(str) => {
                    if str.ends_with(".css") {
                        Some(vec![str.to_owned()])
                    } else {
                        None
                    }
                },
                VecOrString::Vec(files) => {
                    let mut files = files.clone();
                    files.retain(|f| f.ends_with(".css"));
                    Some(files)
                },
            }
        })
    }
}
