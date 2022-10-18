use clap::*;

#[derive(Debug,Parser)]
#[clap(name="kv_clients")]
pub enum ClientArgs {
    Get {
        #[clap(long)]
        key: String,

    },
    Set {
        #[clap(long)]
        key: String,
        #[clap(long)]
        value: String,
    },
    Publish {
        #[clap(long)]
        topic: String,
        value: String,
    },
    Subscribe { 
        #[clap(long)]
        topic: String,
    },
    UnSubscribe {
        #[clap(long)]
        topic: String,
        #[clap(long)]
        id: u8,
    }

}