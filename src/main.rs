use anyhow::Result;
use btleplug::{api::{Central, Manager as _, Peripheral as _, ScanFilter}, platform::{Adapter, Manager}};
use clap::{Parser, Subcommand};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser)]
#[command(name = "rust-ble-scanner")]
#[command(about = "Scan and watch BLE devices", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan { #[arg(long, default_value_t = 10)] duration: u64 },
    Watch { addr: String, #[arg(long, default_value_t = 1)] interval: u64 },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { duration } => scan(duration).await?,
        Commands::Watch { addr, interval } => watch(addr, interval).await?,
    }
    Ok(())
}

async fn get_adapter() -> Result<Adapter> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters.into_iter().next().ok_or_else(|| anyhow::anyhow!("no adapters"))?;
    Ok(adapter)
}

async fn scan(duration: u64) -> Result<()> {
    let adapter = get_adapter().await?;
    adapter.start_scan(ScanFilter::default()).await?;
    sleep(Duration::from_secs(duration)).await;
    let peripherals = adapter.peripherals().await?;
    for p in peripherals {
        let props = p.properties().await?;
        let addr = props.as_ref().map(|x| x.address).map(|a| a.to_string()).unwrap_or_else(|| "?".into());
        let name = props.as_ref().and_then(|x| x.local_name.clone()).unwrap_or_else(|| "".into());
        let rssi = props.as_ref().and_then(|x| x.rssi).map(|v| v.to_string()).unwrap_or_else(|| "".into());
        println!("{}\t{}\t{}", addr, rssi, name);
    }
    adapter.stop_scan().await?;
    Ok(())
}

async fn watch(addr: String, interval: u64) -> Result<()> {
    let adapter = get_adapter().await?;
    adapter.start_scan(ScanFilter::default()).await?;
    sleep(Duration::from_secs(2)).await;
    let mut target = None;
    for p in adapter.peripherals().await? {
        let props = p.properties().await?;
        let a = props.as_ref().map(|x| x.address).map(|a| a.to_string()).unwrap_or_default();
        if a.eq_ignore_ascii_case(&addr) { target = Some(p); break; }
    }
    let p = target.ok_or_else(|| anyhow::anyhow!("device not found"))?;
    loop {
        if let Some(props) = p.properties().await? { if let Some(rssi) = props.rssi { println!("{}", rssi); } }
        sleep(Duration::from_secs(interval)).await;
    }
}
