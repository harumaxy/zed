use anyhow::{anyhow, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::{io::BufReader, StreamExt};
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use smol::fs;
use std::env::consts::ARCH;
use std::{any::Any, path::PathBuf};
use util::async_maybe;
use util::github::latest_github_release;
use util::{github::GitHubLspBinaryVersion, ResultExt};

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
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!("Godot editor must be installed manually"))
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        None
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        None
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }
}
