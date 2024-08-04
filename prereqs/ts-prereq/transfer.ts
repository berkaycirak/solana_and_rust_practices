import {
	Transaction,
	SystemProgram,
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
	sendAndConfirmTransaction,
	PublicKey,
	clusterApiUrl,
} from '@solana/web3.js';

import wallet from './wallet.json';

const from = Keypair.fromSecretKey(new Uint8Array(wallet));

// My WBA public key
const to = new PublicKey(
	'GTfyXsd3pVNvXMbHwJARvoVidzx9LSabTxnvUnyJ1Ppx'
);

const connection = new Connection(
	clusterApiUrl('devnet'),
	'confirmed'
);

const transfer = async () => {
	try {
		// Get balance of dev wallet
		const balance = await connection.getBalance(from.publicKey);

		// Mock transaction to check required fee for all balance transfer in our wallet
		const transaction = new Transaction().add(
			SystemProgram.transfer({
				fromPubkey: from.publicKey,
				toPubkey: to,
				lamports: balance,
			})
		);
		transaction.recentBlockhash = (
			await connection.getLatestBlockhash('confirmed')
		).blockhash;

		transaction.feePayer = from.publicKey;
		// Exact fee calculation to transfer all balance
		const fee =
			(
				await connection.getFeeForMessage(
					transaction.compileMessage(),
					'confirmed'
				)
			).value || 0;
		// Remove our previous transaction instruction
		transaction.instructions.pop();
		// New transaction instruction with exact lamports to empty our wallet
		transaction.add(
			SystemProgram.transfer({
				fromPubkey: from.publicKey,
				toPubkey: to,
				lamports: balance - fee,
			})
		);

		// Sign transaction, broadcast, and confirm
		const signature = await sendAndConfirmTransaction(
			connection,
			transaction,
			[from]
		);
		console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${signature}?cluster=devnet`);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
};

// Transfer

transfer();
