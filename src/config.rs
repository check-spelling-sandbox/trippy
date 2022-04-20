use clap::{ArgEnum, Parser};
use std::time::Duration;

/// The maximum number of hops we allow.
pub const MAX_HOPS: usize = 256;

/// The minimum TUI refresh rate.
pub const TUI_MIN_REFRESH_RATE_MS: Duration = Duration::from_millis(50);

/// The maximum TUI refresh rate.
pub const TUI_MAX_REFRESH_RATE_MS: Duration = Duration::from_millis(1000);

/// The tool mode.
#[derive(Debug, Copy, Clone, ArgEnum)]
pub enum Mode {
    /// Display interactive TUI.
    Tui,
    /// Display a continuous stream of tracing data
    Stream,
    /// Generate a text table report for N cycles.
    Table,
    /// Generate a SCV report for N cycles.
    Csv,
    /// Generate a JSON report for N cycles.
    Json,
}

/// Trace a route to a host and record statistics
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The hostname or IP to scan
    pub hostname: String,

    /// The TTL to start from
    #[clap(long, default_value_t = 1)]
    pub first_ttl: u8,

    /// The maximum number of hops
    #[clap(short = 't', long, default_value_t = 64)]
    pub max_ttl: u8,

    /// The minimum duration of every round
    #[clap(short = 'i', long, default_value = "1s")]
    pub min_round_duration: String,

    /// The maximum duration of every round
    #[clap(short = 'I', long, default_value = "1s")]
    pub max_round_duration: String,

    /// The period of time to wait for additional ICMP responses after the target has responded
    #[clap(short = 'g', long, default_value = "100ms")]
    pub grace_duration: String,

    /// The maximum number of in-flight ICMP echo requests
    #[clap(short = 'U', long, default_value_t = 24)]
    pub max_inflight: u8,

    /// The socket read timeout.
    #[clap(long, default_value = "10ms")]
    pub read_timeout: String,

    /// Preserve the screen on exit
    #[clap(long)]
    pub tui_preserve_screen: bool,

    /// The TUI refresh rate
    #[clap(long, default_value = "100ms")]
    pub tui_refresh_rate: String,

    /// Output mode
    #[clap(arg_enum, short = 'm', long, default_value = "tui")]
    pub mode: Mode,

    /// The number of report cycles to run
    #[clap(short = 'c', long, default_value_t = 10)]
    pub report_cycles: usize,
}
