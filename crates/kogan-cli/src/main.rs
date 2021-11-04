use clap::{Parser};
use kogan::client::KoganClient;

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    env_file: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    GetCategoryList,
    GetOrders,
    GetOrder {
        id: String,
    },
    PostOrderDispatchInfo {
        id: String,
        item_id: String,
        seller_sku: String,
        tracking_number: String,
        shipping_carrier: kogan::order::OrderShippingCarrier,
    },
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
        SubCommand::GetOrders => {
            let res = client.get_orders(Default::default()).await?;
            dbg!(res);
        }
        SubCommand::GetOrder { id } => {
            let res = client.get_order(&id).await?;
            dbg!(res);
        }
        SubCommand::PostOrderDispatchInfo {
            id,
            item_id,
            seller_sku,
            tracking_number,
            shipping_carrier,
        } => {
            use kogan::order::*;
            let res = client
                .post_order_dispatch_info(vec![PostOrderDispatchInfoParams {
                    id,
                    items: vec![OrderDispatchInfoItem {
                        order_item_id: item_id,
                        seller_sku,
                        quantity: 1,
                        shipped_date_utc: chrono::Utc::now(),
                        tracking_number,
                        shipping_carrier,
                    }],
                }])
                .await?;
            dbg!(res);
        }
    }

    Ok(())
}
