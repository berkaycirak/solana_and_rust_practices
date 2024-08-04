import {
	clusterApiUrl,
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import wallet from './wallet.json';

// Our keypair
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
// Connection --->Instead of clusterApiUrl, you can also use 'https://api.devnet.solana.com
const connection = new Connection(
	clusterApiUrl('devnet'),
	'confirmed'
);

(async () => {
	try {
		// We're going to claim 2 devnet SOL tokens
		const txhash = await connection.requestAirdrop(
			keypair.publicKey,
			2 * LAMPORTS_PER_SOL
		);

		console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
