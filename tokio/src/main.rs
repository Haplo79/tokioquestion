use clap::{Parser, ValueEnum};
use std::time::Instant;

#[derive(Debug, Parser)]
struct CommandLineInterface {
    /// Which runtime to use.
    #[clap(long, value_enum)]
    pub runtime: TokioRuntime,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum TokioRuntime {
    Current,
    Multi,
}

fn main() -> anyhow::Result<()> {
    let cli = CommandLineInterface::parse();
    println!("{:?}", cli);

    match cli.runtime {
        TokioRuntime::Multi => tokio_multi_thread(),
        TokioRuntime::Current => tokio_current_thread(),
    }
}

async fn run() -> anyhow::Result<()> {
    let local = tokio::task::LocalSet::new();
    local.spawn_local(task());
    local.await;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn tokio_current_thread() -> anyhow::Result<()> {
    run().await
}

#[tokio::main]
async fn tokio_multi_thread() -> anyhow::Result<()> {
    run().await
}

struct Recorder {
    count: usize,
    last_instant: Instant,
}

impl Default for Recorder {
    fn default() -> Self {
        Self {
            count: 0,
            last_instant: Instant::now(),
        }
    }
}

const BATCH_SIZE: usize = 5000000;

impl Recorder {
    fn iteration(&mut self) {
        self.count += 1;

        if self.count == BATCH_SIZE {
            let now = Instant::now();
            self.count = 0;
            let duration = now.duration_since(self.last_instant);
            println!("{} calls took {} millis", BATCH_SIZE, duration.as_millis());
            self.last_instant = now;
        }
    }
}

async fn task() {
    let mut recorder = Recorder::default();

    loop {
        recorder.iteration();
        tokio::task::yield_now().await;
    }
}
