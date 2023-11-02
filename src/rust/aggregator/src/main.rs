use std::{
    fmt::Display,
    sync::Arc,
    time::{Duration, SystemTime},
};

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use data::{leaderboards::Leaderboards, user_history::UserHistory, Pipeline};
use sqlx::PgPool;
use tokio::{sync::Mutex, task::JoinSet};
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// If set, no pipeline will be included by default.
    #[arg(short, long)]
    no_default: bool,

    /// Exclusion list. Pipelines specified here will not be executed. Ignored if --no-default passed.
    #[arg(short, long)]
    exclude: Option<Vec<Pipelines>>,

    /// Inclusion list. Pipelines specified here will be executed. Ignored if --no-default is not passed.
    #[arg(short, long, value_enum)]
    include: Option<Vec<Pipelines>>,

    /// Database URL.
    #[arg(short, long)]
    database_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pipelines {
    UserHistory,
    Leaderboards,
}

impl ValueEnum for Pipelines {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.to_string()))
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::UserHistory, Self::Leaderboards]
    }
}

impl Display for Pipelines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

mod data;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    tracing::info!("Started up.");
    dotenvy::dotenv().ok();
    let args: Args = Args::parse();

    let database_url = std::env::var("DATABASE_URL")
        .or_else(|_| -> Result<String> {
            args.database_url
                .ok_or(anyhow!("No database URL was provided."))
        })
        .expect("DATABASE_URL should be set");

    let pool = PgPool::connect(&database_url).await?;

    tracing::info!("Connected to DB.");

    let default_interval = Duration::from_secs(5);

    let mut data: Vec<Arc<Mutex<dyn Pipeline + Send + Sync>>> = vec![];

    let pipelines = if args.no_default {
        (args
            .include
            .ok_or(anyhow!("No data is included and --no-default is set."))?)
        .clone()
    } else {
        let mut x = vec![Pipelines::UserHistory, Pipelines::Leaderboards];
        x.append(&mut args.include.unwrap_or(vec![]));
        x.dedup();
        x
    };

    for pipeline in pipelines {
        match pipeline {
            Pipelines::UserHistory => {
                data.push(Arc::new(Mutex::new(UserHistory::new(pool.clone()))));
            }
            Pipelines::Leaderboards => {
                data.push(Arc::new(Mutex::new(Leaderboards::new(pool.clone()))));
            }
        }
    }

    let mut handles = JoinSet::new();

    for data in data {
        handles.spawn(async move {
            let mut data = data.lock().await;

            tracing::info!(
                "[{}] Starting process & save (historical).",
                data.model_name()
            );
            data.process_and_save_historical_data().await?;
            tracing::info!(
                "[{}] Finished process & save (historical).",
                data.model_name()
            );

            loop {
                let interval = data.poll_interval().unwrap_or(default_interval);

                tokio::time::sleep(interval).await;

                if data.ready() {
                    tracing::info!("[{}] Starting process & saving.", data.model_name());
                    let start = SystemTime::now();
                    data.process_and_save().await?;
                    let time = start
                        .elapsed()
                        .unwrap_or(Duration::from_secs(0))
                        .as_millis();
                    tracing::info!(
                        "[{}] Finished process & saving in {}ms.",
                        data.model_name(),
                        time
                    );
                } else {
                    tracing::info!("[{}] Data is not ready.", data.model_name());
                }
            }

            #[allow(unreachable_code)]
            Ok::<(), anyhow::Error>(())
        });
    }

    while let Some(res) = handles.join_next().await {
        res??;
    }

    Ok(())
}
