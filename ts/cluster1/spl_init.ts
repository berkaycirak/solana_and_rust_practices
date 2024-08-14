import { Keypair, Connection, Commitment } from '@solana/web3.js';
import { createMint } from '@solana/spl-token';
import wallet from './wallet/wba-wallet.json';

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection(
	'http://127.0.0.1:8899',
	commitment
);
const walletPk = [
	167, 43, 92, 26, 154, 54, 14, 155, 110, 56, 184, 186, 226, 102, 149,
	25, 162, 176, 188, 114, 62, 177, 55, 76, 12, 132, 137, 181, 114,
	204, 171, 203, 108, 121, 143, 184, 24, 20, 65, 111, 39, 230, 18, 62,
	125, 51, 12, 30, 200, 115, 108, 197, 244, 12, 175, 44, 115, 14, 80,
	243, 168, 8, 36, 160,
];
const signer = Keypair.fromSecretKey(new Uint8Array(walletPk));

(async () => {
	try {
		// Start here
		const mint = await createMint(
			connection,
			signer,
			signer.publicKey,
			null,
			6
		);
		console.log('Your mint address ---------->', mint.toBase58());
	} catch (error) {
		console.log(`Oops, something went wrong: ${error}`);
	}
})();
