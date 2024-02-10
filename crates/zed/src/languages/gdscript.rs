use anyhow::{anyhow, Result};
use async_trait::async_trait;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf};

pub struct GodotLspAdapter;

#[async_trait]
impl LspAdapter for GodotLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("gdscript".into())
    }

    fn short_name(&self) -> &'static str {
        "gd"
    }

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _version: Box<dyn 'static + Send + Any>,
        _container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!("Godot editor must be installed manually"))
    }

    async fn cached_server_binary(
        &self,
        _container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        None
    }

    async fn installation_test_binary(
        &self,
        _container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        None
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }
}
