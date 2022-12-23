use crate::prelude::*;

pub trait SaveLoad: AsRef<Path> {
    fn load_raw(&self) -> Result<Vec<u8>> { Ok(std::fs::read(self)?) }

    fn load_file(&self) -> Result<String> { Ok(std::fs::read_to_string(self)?) }

    fn load_toml<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let contents = self.load_file()?;
        let value = toml::from_str::<T>(&contents)?;
        Ok(value)
    }

    fn load_ron<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let contents = self.load_file()?;
        let value = ron::from_str::<T>(&contents)?;
        Ok(value)
    }

    fn save_file(&self, contents: &str) -> Result<()> {
        if let Some(dir) = self.as_ref().parent() {
            create_dir_all(dir)?;
        }
        let file = File::create(self)?;
        let mut file = BufWriter::new(file);
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    fn to_toml<T: Serialize>(&self, value: &T) -> Result<()> {
        let contents = toml::to_string_pretty(value)?;
        self.save_file(&contents)?;
        Ok(())
    }

    fn to_ron<T: Serialize>(&self, value: &T) -> Result<()> {
        let contents = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default())?;
        self.save_file(&contents)?;
        Ok(())
    }

    fn to_json<T: Serialize>(&self, value: &T) -> Result<()> {
        let contents = serde_json::to_string_pretty(value)?;
        self.save_file(&contents)?;
        Ok(())
    }

    fn path_string(&self) -> String {
        if let Ok(s) = AsRef::<Path>::as_ref(self).canonicalize() {
            if let Some(s) = s.to_str() {
                s.to_string()
            } else {
                "UNABLE/TO/CONVERT/PATH/TO/STRING".to_string()
            }
        } else {
            "INVALID/PATH".to_string()
        }
    }
}

impl<T: AsRef<Path>> SaveLoad for T {}
