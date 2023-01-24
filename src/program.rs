use {
    solana_sdk::{signature::{Keypair, Signer,},},
    std::{
        sync::{
            atomic::{AtomicBool, AtomicU64, Ordering},
            Arc,
        },
        thread,
        time::Instant,
        error::Error,
    },
    mongodb::{Client, Collection, options::{ClientOptions, ResolverConfig, FindOneAndUpdateOptions}, bson::doc},
    tokio::runtime::Runtime,
};

#[derive(serde::Deserialize)]
struct Document {
    _id: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let num_threads: usize = num_cpus::get();
    println!("num threads: {}", num_threads);
    let attempts = Arc::new(AtomicU64::new(1));
    let start = Instant::now();
    let done = Arc::new(AtomicBool::new(false));
    let thread_handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let done = done.clone();
            let attempts = attempts.clone();
            thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                let _ = rt.block_on(upload(done, attempts, start));
            })
        })
        .collect();

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    Ok(())
}

async fn upload(done: Arc<AtomicBool>, attempts: Arc<AtomicU64>, start: Instant) -> Result<(), Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017";
    let options = ClientOptions::parse_with_resolver_config(
        &client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .build();
    loop {
        if done.load(Ordering::Relaxed) {
            break;
        }
        let collection: Collection<Document> = client.database("solana").collection("keygen");
        let attempts = attempts.fetch_add(1, Ordering::Relaxed);
        let (keypair, _) = (Keypair::new(), "".to_string());
        let pubkey = bs58::encode(keypair.pubkey()).into_string();
        let privkey = keypair.to_base58_string();
        collection.find_one_and_update(doc! { "_id": pubkey }, doc! { "$set": { "k": privkey } }, Some(options.clone())).await?;
        if attempts % 1_000_000 == 0 {
            println!(
                "Searched {} keypairs in {}s.",
                attempts,
                start.elapsed().as_secs(),
            );
        }
    }
    return Ok(())
}