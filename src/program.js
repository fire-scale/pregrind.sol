const web3 = require('@solana/web3.js');
const bs58 = require('bs58');

// TODO: create a descending index
return;

let attempts = 0;
const start = Date.now();
while (true) {
    attempts++;
    const keypair = new web3.Keypair();
    const pubkey = keypair.publicKey.toBase58();
    if (attempts % 100_000 === 0) {
        const now = Date.now();
        const diff = Math.round((now - start) / 1000);
        console.log(`Searched ${attempts} keypairs in ${diff}s.`);
        console.log(`${pubkey}: ${bs58.encode(keypair.secretKey)}`);
    }
}