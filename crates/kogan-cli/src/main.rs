use clap::{AppSettings, Clap};
use kogan::client::KoganClient;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    env_file: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    GetCategoryList,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts: Opts = Opts::parse();

    if let Some(c) = opts.env_file {
        dotenv::from_filename(&c).unwrap();
        println!("using env file: {}", c);
    } else {
        dotenv::dotenv().unwrap();
    }

    let client = KoganClient::from_env()?;

    match opts.subcmd {
        SubCommand::GetCategoryList => {
            let res = client.get_category_list(None).await?;
            dbg!(res);
        }
    }

    Ok(())
}