use zed_extension_api as zed;

struct TechniqueExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for TechniqueExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }
}

zed::register_extension!(TechniqueExtension);
