#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
// The `chainweb` feature must be enabled to use this crate.
#![cfg(feature = "chainweb")]

use clap::Parser;
use reth_node_builder::{engine_tree_config::TreeConfig, EngineNodeLauncher};
//use reth_node_optimism::{args::RollupArgs, node::OptimismAddOns, OptimismNode};
use reth_chainweb_cli::{chainspec::CwChainSpecParser, Cli};

//use reth_provider::providers::BlockchainProvider2;

#[global_allocator]
static ALLOC: reth_cli_util::allocator::Allocator = reth_cli_util::allocator::new_allocator();

fn main() {
    reth_cli_util::sigsegv_handler::install();

    // Enable backtraces unless a RUST_BACKTRACE value has already been explicitly provided.
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    if let Err(err) = Cli::<CwChainSpecParser>::parse().run(|builder, args| async move {
        /*
        let enable_engine2 = rollup_args.experimental;
        let sequencer_http_arg = rollup_args.sequencer_http.clone();
        match enable_engine2 {
            true => {
                let engine_tree_config = TreeConfig::default()
                    .with_persistence_threshold(rollup_args.persistence_threshold)
                    .with_memory_block_buffer_target(rollup_args.memory_block_buffer_target);
                let handle = builder
                    .with_types_and_provider::<OptimismNode, BlockchainProvider2<_>>()
                    .with_components(OptimismNode::components(rollup_args))
                    .with_add_ons::<OptimismAddOns>()
                    .extend_rpc_modules(move |ctx| {
                        // register sequencer tx forwarder
                        if let Some(sequencer_http) = sequencer_http_arg {
                            ctx.registry
                                .eth_api()
                                .set_sequencer_client(SequencerClient::new(sequencer_http))?;
                        }

                        Ok(())
                    })
                    .launch_with_fn(|builder| {
                        let launcher = EngineNodeLauncher::new(
                            builder.task_executor().clone(),
                            builder.config().datadir(),
                            engine_tree_config,
                        );
                        builder.launch_with(launcher)
                    })
                    .await?;

                handle.node_exit_future.await
            }
            false => {
                let handle = builder
                    .node(OptimismNode::new(rollup_args.clone()))
                    .extend_rpc_modules(move |ctx| {
                        // register sequencer tx forwarder
                        if let Some(sequencer_http) = sequencer_http_arg {
                            ctx.registry
                                .eth_api()
                                .set_sequencer_client(SequencerClient::new(sequencer_http))?;
                        }

                        Ok(())
                    })
                    .launch()
                    .await?;

                handle.node_exit_future.await
            }
        }*/

        println!("Hello Reth!");
        Ok(())
    }) {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
