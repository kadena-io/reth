//! Chainweb Reth CLI implementation.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
// The `chainweb` feature must be enabled to use this crate.
// #![cfg(feature = "chainweb")]

/// Chainweb chain specification parser.
pub mod chainspec;
pub mod commands;

use std::{ffi::OsString, fmt, sync::Arc};

use chainspec::CwChainSpecParser;
use clap::{command, value_parser, Parser};
use commands::Commands;
use futures_util::Future;
use reth_chainspec::ChainSpec;
use reth_cli::chainspec::ChainSpecParser;
use reth_cli_commands::node::NoArgs;
use reth_cli_runner::CliRunner;
use reth_db::DatabaseEnv;
//use reth_evm_optimism::OpExecutorProvider;
use reth_node_builder::{NodeBuilder, WithLaunchContext};
use reth_node_core::{
    args::LogArgs,
    version::{LONG_VERSION, SHORT_VERSION},
};
//use reth_node_optimism::OptimismNode;
use reth_tracing::FileWorkerGuard;
use tracing::info;

/// The main op-reth cli interface.
///
/// This is the entrypoint to the executable.
#[derive(Debug, Parser)]
#[command(author, version = SHORT_VERSION, long_version = LONG_VERSION, about = "Reth", long_about = None)]
pub struct Cli<
    Spec: ChainSpecParser<ChainSpec = ChainSpec> = CwChainSpecParser,
    Ext: clap::Args + fmt::Debug = NoArgs,
> {
    /// The command to run
    #[command(subcommand)]
    command: Commands<Spec, Ext>,

    /// The chain this node is running.
    ///
    /// Possible values are either a built-in chain or the path to a chain specification file.
    #[arg(
        long,
        value_name = "CHAIN_OR_PATH",
        long_help = Spec::help_message(),
        default_value = Spec::SUPPORTED_CHAINS[0],
        value_parser = Spec::parser(),
        global = true,
    )]
    chain: Arc<Spec::ChainSpec>,

    /// Add a new instance of a node.
    ///
    /// Configures the ports of the node to avoid conflicts with the defaults.
    /// This is useful for running multiple nodes on the same machine.
    ///
    /// Max number of instances is 200. It is chosen in a way so that it's not possible to have
    /// port numbers that conflict with each other.
    ///
    /// Changes to the following port numbers:
    /// - `DISCOVERY_PORT`: default + `instance` - 1
    /// - `AUTH_PORT`: default + `instance` * 100 - 100
    /// - `HTTP_RPC_PORT`: default - `instance` + 1
    /// - `WS_RPC_PORT`: default + `instance` * 2 - 2
    #[arg(long, value_name = "INSTANCE", global = true, default_value_t = 1, value_parser = value_parser!(u16).range(..=200))]
    instance: u16,

    #[command(flatten)]
    logs: LogArgs,
}

impl Cli {
    /// Parsers only the default CLI arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Parsers only the default CLI arguments from the given iterator
    pub fn try_parse_args_from<I, T>(itr: I) -> Result<Self, clap::error::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::try_parse_from(itr)
    }
}

impl<Spec, Ext> Cli<Spec, Ext>
where
    Spec: ChainSpecParser<ChainSpec = ChainSpec>,
    Ext: clap::Args + fmt::Debug,
{
    /// Execute the configured cli command.
    ///
    /// This accepts a closure that is used to launch the node via the
    /// [`NodeCommand`](reth_cli_commands::node::NodeCommand).
    pub fn run<L, Fut>(mut self, launcher: L) -> eyre::Result<()>
    where
        L: FnOnce(WithLaunchContext<NodeBuilder<Arc<DatabaseEnv>, ChainSpec>>, Ext) -> Fut,
        Fut: Future<Output = eyre::Result<()>>,
    {
        // add network name to logs dir
        self.logs.log_file_directory =
            self.logs.log_file_directory.join(self.chain.chain.to_string());

        let _guard = self.init_tracing()?;
        info!(target: "reth::cli", "Initialized tracing, debug log directory: {}", self.logs.log_file_directory);

        let runner = CliRunner::default();
        match self.command {
            Commands::Node(command) => {
                runner.run_command_until_exit(|ctx| command.execute(ctx, launcher))
            }
            Commands::Init(command) => {
                // runner.run_blocking_until_ctrl_c(command.execute::<OptimismNode>())
                Ok(())
            }
            /*
            Commands::InitState(command) => {
                runner.run_blocking_until_ctrl_c(command.execute::<OptimismNode>())
            }
            Commands::ImportOp(command) => {
                runner.run_blocking_until_ctrl_c(command.execute::<OptimismNode>())
            }
            Commands::ImportReceiptsOp(command) => {
                runner.run_blocking_until_ctrl_c(command.execute::<OptimismNode>())
            }
            */
            Commands::DumpGenesis(command) => runner.run_blocking_until_ctrl_c(command.execute()),

            Commands::Db(command) => {
                Ok(())
                //runner.run_blocking_until_ctrl_c(command.execute::<OptimismNode>())
            }
            Commands::Stage(command) =>
            //runner.run_command_until_exit(|ctx| {
            {
                Ok(())
                //command.execute::<OptimismNode, _, _>(ctx, OpExecutorProvider::optimism)
                //}),
            }

            Commands::P2P(command) => runner.run_until_ctrl_c(command.execute()),

            Commands::Config(command) => runner.run_until_ctrl_c(command.execute()),

            Commands::Recover(command) => {
                Ok(())
                //runner.run_command_until_exit(|ctx| command.execute::<OptimismNode>(ctx))
            }
            Commands::Prune(command) => {
                Ok(())
                //runner.run_until_ctrl_c(command.execute::<OptimismNode>())}
            }
        }
    }

    /// Initializes tracing with the configured options.
    ///
    /// If file logging is enabled, this function returns a guard that must be kept alive to ensure
    /// that all logs are flushed to disk.
    pub fn init_tracing(&self) -> eyre::Result<Option<FileWorkerGuard>> {
        let guard = self.logs.init_tracing()?;
        Ok(guard)
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;
    use reth_chainweb_chainspec::CW_DEV;
    use reth_cli_commands::NodeCommand;

    #[test]
    fn parse_dev() {
        let cmd: NodeCommand = NodeCommand::parse_from(["cw-reth", "--dev"]);
        let chain = CW_DEV.clone();
        assert_eq!(cmd.chain.chain, chain.chain);
        assert_eq!(cmd.chain.genesis_hash, chain.genesis_hash);
        assert_eq!(
            cmd.chain.paris_block_and_final_difficulty,
            chain.paris_block_and_final_difficulty
        );
        assert_eq!(cmd.chain.hardforks, chain.hardforks);

        assert!(cmd.rpc.http);
        assert!(cmd.network.discovery.disable_discovery);

        assert!(cmd.dev.dev);
    }
}
