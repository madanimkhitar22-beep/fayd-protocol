//! Fayd Node — Decentralized Compute Protocol
//!
//! A lightweight node for contributing idle compute to the Fayd network.
//! Built with simplicity, verifiability, and accessibility in mind.
//!
//! Repository: https://github.com/madanimkhitar22-beep/fayd-protocol

use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::process;
use std::thread;
use std::time::Duration;
use tracing::{info, warn};
use tracing_subscriber::{fmt, EnvFilter};

// ============================================================================
// CLI Arguments
// ============================================================================

/// Fayd Node — Decentralized Compute Protocol
///
/// Contribute idle compute to the Fayd network.
/// Runs on phones, laptops, servers, and IoT devices.
#[derive(Parser, Debug)]
#[command(name = "fayd-node")]
#[command(version = "0.1.0")]
#[command(about = "Fayd Protocol Node", long_about = None)]
struct Args {
    /// Node display name
    #[arg(short, long, default_value = "anonymous")]
    name: String,

    /// Maximum CPU usage percentage (1-100)
    #[arg(long, default_value = "50", value_parser = clap::value_parser!(u8).range(1..=100))]
    max_cpu: u8,

    /// Maximum memory usage in MB
    #[arg(long, default_value = "512")]
    max_memory_mb: u64,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

// ============================================================================
// Node Identity and Metadata
// ============================================================================
/// Node status enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
enum NodeStatus {
    Idle,
    Busy,
    Verifying,
    Offline,
}

impl std::fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeStatus::Idle => write!(f, "idle"),
            NodeStatus::Busy => write!(f, "busy"),
            NodeStatus::Verifying => write!(f, "verifying"),
            NodeStatus::Offline => write!(f, "offline"),
        }
    }
}

/// Node information structure
#[derive(Debug, Serialize, Deserialize)]
struct NodeInfo {
    /// Unique node identifier
    id: String,
    /// Human-readable name
    name: String,
    /// Protocol version
    version: String,
    /// Current status
    status: NodeStatus,
    /// Resource limits
    max_cpu: u8,
    max_memory_mb: u64,
}

impl NodeInfo {
    fn new(args: &Args) -> Self {
        Self {
            id: generate_node_id(),
            name: args.name.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            status: NodeStatus::Idle,
            max_cpu: args.max_cpu,
            max_memory_mb: args.max_memory_mb,
        }
    }
}

// ============================================================================// Utility Functions
// ============================================================================

/// Generate a unique node identifier
/// Format: fx_<16 hex characters>
fn generate_node_id() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 8] = rng.gen();
    format!("fx_{}", hex::encode(bytes))
}

/// Initialize tracing subscriber for structured logging
fn init_tracing(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("fayd_node=debug,info")
    } else {
        EnvFilter::from_default_env()
    };

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_names(false)
        .init();
}

/// Display node information banner
fn display_banner(node: &NodeInfo) {
    info!("🌊 Fayd Node Starting");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("Node ID     : {}", node.id);
    info!("Name        : {}", node.name);
    info!("Version     : {}", node.version);
    info!("Status      : {}", node.status);
    info!("Max CPU     : {}%", node.max_cpu);
    info!("Max Memory  : {} MB", node.max_memory_mb);
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

// ============================================================================
// Node Runtime
// ============================================================================

/// Run the node main loop
fn run_node(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Create node info
    let node = NodeInfo::new(&args);

    // Display banner
    display_banner(&node);
    // Phase 0: Simulated connection to test mesh
    info!("Connecting to Fayd test mesh...");
    thread::sleep(Duration::from_millis(500));
    info!("✓ Connected to Fayd test mesh");
    info!("✓ Ready to receive tasks");
    info!("");
    info!("Press Ctrl+C to stop the node.");
    info!("");

    // Main idle loop
    // In Phase 0, the node simply waits for tasks
    // Future phases will implement:
    // - Task reception and execution
    // - Peer discovery and coordination
    // - Verification and proof generation
    loop {
        // Heartbeat every 60 seconds
        thread::sleep(Duration::from_secs(60));
        // In future phases, this will:
        // - Check for incoming tasks
        // - Report resource availability
        // - Maintain peer connections
    }
}

// ============================================================================
// Entry Point
// ============================================================================

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Initialize logging
    init_tracing(args.verbose);

    // Run node
    if let Err(e) = run_node(args) {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}
