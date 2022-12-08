use super::*;

impl Project {
    pub fn edition(&mut self, major: u64, minor: u64) {
        self.edition = Version { major, minor, patch: 0, pre: Default::default(), build: Default::default() }
    }
    pub fn version(&mut self, v: &str) -> QResult {
        self.version = Version::from_str(v)?;
        Ok(())
    }
}
