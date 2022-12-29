use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    InstallFramework,
    Decode,
    Build,
    PublicizeResources,
    EmptyFrameworkDir,
    ListFrameworks,
}

#[derive(Args)]
pub struct DecodeOptions {
    #[arg(short = 's', long = "no-src")]
    pub no_src: bool,
    #[arg(long = "only-main-classes")]
    pub only_main_classes: bool,
    #[arg(short, long)]
    pub debug: bool,
    #[arg(short = 'b', long = "no-debug-info")]
    pub no_baksmali_debug_info: bool,
    #[arg(short = 't', long = "frame-tag")]
    pub frame_tag: Option<String>,
    #[arg(short = 'f', long = "force")]
    pub force_delete: bool,
    #[arg(short = 'r', long = "no-res")]
    pub no_resources: bool,
    #[arg(long = "force-manifest")]
    pub force_manifest: bool,
    #[arg(long = "no-assets")]
    pub no_assets: bool,
    #[arg(short = 'k', long = "keep-broken-res")]
    pub keep_broken_resources: bool,
    #[arg(short = 'p', long = "frame-path")]
    pub framework_path: Option<String>,
    #[arg(short = 'm', long = "match-original")]
    pub match_original: bool,
    #[arg(short = 'a', long = "api-level")]
    pub api_level: Option<u32>,
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,
}
