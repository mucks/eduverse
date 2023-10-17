use anyhow::{anyhow, Context, Result};
use web_sys::Storage;

pub const DEBUG: bool = cfg!(debug_assertions);

pub struct LocalStorage {
    inner: web_sys::Storage,
}

impl LocalStorage {
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: local_storage()?,
        })
    }

    pub fn get(&self, key: &str) -> Result<String> {
        self.inner
            .get_item(key)
            .map_err(|e| anyhow!("{:?}", e))?
            .with_context(|| anyhow!("could not find {key}"))
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        Ok(self
            .inner
            .set_item(key, value)
            .map_err(|e| anyhow!("{:?}", e))?)
    }
}

fn local_storage() -> Result<Storage> {
    Ok(web_sys::window()
        .context("no global `window` exists")?
        .local_storage()
        .map_err(|e| anyhow!("no local storage: {e:?}"))?
        .context("could not get local storage")?)
}
