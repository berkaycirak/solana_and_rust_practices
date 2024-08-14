import {
	Commitment,
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
} from '@solana/web3.js';
// import wallet from '../wba-wallet.json';
import {
	getOrCreateAssociatedTokenAccount,
	transfer,
} from '@solana/spl-token';

const walletPk = [
	167, 43, 92, 26, 154, 54, 14, 155, 110, 56, 184, 186, 226, 102, 149,
	25, 162, 176, 188, 114, 62, 177, 55, 76, 12, 132, 137, 181, 114,
	204, 171, 203, 108, 121, 143, 184, 24, 20, 65, 111, 39, 230, 18, 62,
	125, 51, 12, 30, 200, 115, 108, 197, 244, 12, 175, 44, 115, 14, 80,
	243, 168, 8, 36, 160,
];

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(walletPk));

//Create a Solana devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection(
	'http://127.0.0.1:8899',
	commitment
);

// Mint address
const mint = new PublicKey(
	'DxhNFBqUkQr2DJ964aovMJBFwCFoe1vr8WDzjEyrUy7H'
);

// Recipient address
// const to = new PublicKey('<receiver address>');

(async () => {
	try {
		// Get the token account of the fromWallet address, and if it does not exist, create it
		const fromATA = await getOrCreateAssociatedTokenAccount(
			connection,
			keypair,
			mint,
			keypair.publicKey
		);
		// Get the token account of the toWallet address, and if it does not exist, create it
		const toATA = await getOrCreateAssociatedTokenAccount(
			connection,
			keypair,
			mint,
			keypair.publicKey
		);

		// Transfer the new token to the "toTokenAccount" we just created
		const signature = await transfer(
			connection,
			keypair,
			fromATA.address,
			toATA.address,
			keypair,
			1e6
		);

		console.log(signature);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
