use crate::prelude::*;

pub trait SaveLoad: AsRef<Path> {
    fn load_file(&self) -> Result<String> {
        let file = File::open(self)?;
        let mut file = BufReader::new(file);
        let mut contents = String::new();
        let _bytes_read = file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn from_toml<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let contents = self.load_file()?;
        let value = toml::from_str::<T>(&contents)?;
        Ok(value)
    }

    fn from_ron<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let contents = self.load_file()?;
        let value = ron::from_str::<T>(&contents)?;
        Ok(value)
    }

    fn save_file(&self, contents: &str) -> Result<()> {
        create_dir_all(self)?;
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
}

impl<T: AsRef<Path>> SaveLoad for T {}
