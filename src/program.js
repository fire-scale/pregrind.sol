const web3 = require('@solana/web3.js');
const bs58 = require('bs58');

// 4amLeuKGKMn1jQtrX5LvAdxCF3dASiWZpn2msqJbzbm9
const key = bs58.decode("2796zULToaxixe1RvP1vNLJStnkRWPAPRqv87yrFwRgbTSRi2dM4JVKRZwhXgCCuKUKWForLt3TUmEGPMvZwaJ6b");
const pair = web3.Keypair.fromSecretKey(key);
console.log(`public is ${pair.publicKey.toBase58()}`);
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