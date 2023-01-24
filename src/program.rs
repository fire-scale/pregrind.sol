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
};

#[derive(serde::Deserialize)]
struct Document {
    _id: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // DB

    let client_uri = "mongodb://localhost:27017";
    let options = ClientOptions::parse_with_resolver_config(
        &client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;
    // Print the databases in our MongoDB cluster:
    let collection: Collection<Document> = client.database("solana").collection("keygen");
    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .build();
    collection.find_one_and_update(doc! { "_id": "testing2" }, doc! { "$set": { "k": "another21" } }, Some(options)).await?;

    // GRIND

    let num_threads: usize = num_cpus::get();
    println!("num threads: {}", num_threads);
    let attempts = Arc::new(AtomicU64::new(1));
    let start = Instant::now();
    let done = Arc::new(AtomicBool::new(false));

    let thread_handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let done = done.clone();
            let attempts = attempts.clone();

            thread::spawn(move || loop {
                if done.load(Ordering::Relaxed) {
                    break;
                }
                let attempts = attempts.fetch_add(1, Ordering::Relaxed);
                let (keypair, _) = (Keypair::new(), "".to_string());
                let pubkey = bs58::encode(keypair.pubkey()).into_string();
                if attempts % 1_000_000 == 0 {
                    println!(
                        "Searched {} keypairs in {}s.",
                        attempts,
                        start.elapsed().as_secs(),
                    );
                    println!("{}: {}", pubkey, &keypair.to_base58_string());
                }
            })
        })
        .collect();

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    Ok(())
}
