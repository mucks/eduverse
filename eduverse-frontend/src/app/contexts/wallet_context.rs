use std::{cell::RefCell, str::FromStr};

use anyhow::{anyhow, Context, Result};
use log::debug;
use solana_sdk::{pubkey::Pubkey, transaction::Transaction, wasm_bindgen};
use wasm_bindgen::JsValue;

use crate::util::LocalStorage;

#[wasm_bindgen(module = "/js/phantom.js")]
extern "C" {
    fn tx_from_buffer(buf: &[u8]) -> JsValue;
}

fn reflect_get(target: &JsValue, key: &JsValue) -> Result<JsValue> {
    let result = js_sys::Reflect::get(target, key).map_err(|e| anyhow!("{:?}", e))?;
    debug!("reflect_get: {:?}", result);
    Ok(result)
}

#[derive(Debug, Clone, PartialEq)]
pub struct WalletContext {
    pubkey: RefCell<Option<Pubkey>>,
}

impl WalletContext {
    pub fn new() -> Self {
        let pubkey = Self::get_pubkey().ok();
        Self {
            pubkey: RefCell::new(pubkey),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.pubkey.borrow().is_some()
    }

    pub fn pubkey(&self) -> Option<Pubkey> {
        self.pubkey.borrow().clone()
    }

    pub fn pubkey_str(&self) -> String {
        let pk = self.pubkey();
        match pk {
            Some(p) => p.to_string(),
            None => "".to_string(),
        }
    }

    pub async fn connect(&self) -> Result<()> {
        debug!("connect_to_wallet");
        let window = web_sys::window().context("could not get window")?;
        if let Some(solana) = window.get("solana") {
            let is_phantom = reflect_get(&*solana, &wasm_bindgen::JsValue::from_str("isPhantom"))?;

            if is_phantom == JsValue::from(true) {
                let connect_str = wasm_bindgen::JsValue::from_str("connect");
                let connect: js_sys::Function =
                    js_sys::Reflect::get(&*solana, &connect_str).unwrap().into();
                log::debug!("{:?}", connect.to_string());
                let resp = connect.call0(&solana).unwrap();
                let promise = js_sys::Promise::resolve(&resp);

                let result = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                log::debug!("{:?}", result);
                let pubkey_str = wasm_bindgen::JsValue::from_str("publicKey");
                let pubkey_obj: js_sys::Object =
                    js_sys::Reflect::get(&result, &pubkey_str).unwrap().into();

                let bn_str = wasm_bindgen::JsValue::from_str("toString");
                let to_string_fn: js_sys::Function =
                    js_sys::Reflect::get(&pubkey_obj, &bn_str).unwrap().into();

                log::debug!("pubkey_obj: {:?}", to_string_fn.call0(&pubkey_obj));

                let pubkey = to_string_fn.call0(&pubkey_obj).unwrap();
                let public_key = Pubkey::from_str(&pubkey.as_string().unwrap()).unwrap();
                *self.pubkey.borrow_mut() = Some(public_key);
                Self::save_pubkey(public_key).unwrap();

                log::debug!("pubkey: {:?}", public_key);
            }

            debug!("isPhantom: {:?}", is_phantom);
        }
        Ok(())
    }

    fn save_pubkey(pubkey: Pubkey) -> Result<()> {
        let storage = LocalStorage::new()?;
        storage.set("pubkey", &pubkey.to_string())?;
        Ok(())
    }

    fn get_pubkey() -> Result<Pubkey> {
        let storage = LocalStorage::new()?;
        let pubkey_str = storage.get("pubkey")?;
        let pubkey = Pubkey::from_str(&pubkey_str)?;
        Ok(pubkey)
    }

    pub async fn sign_and_send_transaction(&self, tx: Transaction) -> Result<()> {
        let solana = web_sys::window()
            .context("could not get window")?
            .get("solana")
            .context("could not get solana")?;
        let sign_tx_str = wasm_bindgen::JsValue::from_str("signAndSendTransaction");
        let sign_tx_method: js_sys::Function = reflect_get(&*solana, &sign_tx_str)?.into();

        let tx_bytes = bincode::serialize(&tx)?;

        let tx_js = tx_from_buffer(&tx_bytes);

        let resp = sign_tx_method
            .call1(&solana, &tx_js)
            .map_err(|err| anyhow!("error signing and sending transaction: {:?}", err))?;

        let promise = js_sys::Promise::resolve(&resp);
        let result = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(|err| anyhow!("error signing and sending transaction: {:?}", err))?;

        log::debug!("result: {:?}", result);

        Ok(())
    }
}
