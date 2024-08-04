import { Keypair } from '@solana/web3.js';

// Generate a random new keypair
let keypair = Keypair.generate();
console.log(
	`You\'ve generated a new Solana wallet, Public Key--------> ${keypair.publicKey.toBase58()}`
);
console.log(
	`You\'ve generated a new Solana wallet, Private Key--------> ${keypair.secretKey}`
);
