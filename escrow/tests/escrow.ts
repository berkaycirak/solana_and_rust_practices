import * as anchor from '@coral-xyz/anchor';
import { Program, BN } from '@coral-xyz/anchor';
import { Escrow } from '../target/types/escrow';
import { PublicKey } from '@solana/web3.js';

import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
describe('escrow', async () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	try {
	} catch (error) {
		console.log(`Oops, something went wrong: ${error}`);
	}
	const maker = new PublicKey(
		'8JSYNX179Meg1htpjkTZJybhXe62rX2HS4UWiRLszk6B'
	);
	const mint = new PublicKey(
		'DxhNFBqUkQr2DJ964aovMJBFwCFoe1vr8WDzjEyrUy7H'
	);

	const program = anchor.workspace.Escrow as Program<Escrow>;

	console.log('hello');

	it('Is initialized!', async () => {
		const seed = new BN(0); // if it is first escrow, make it 0.
		const receive = new BN(100); //mint_b
		const deposit = new BN(200); // mint_a
		const tx = await program.methods
			.make(seed, receive, deposit)
			.accounts({
				maker: maker,
				mintA: mint,
				mintB: mint,
				tokenProgram: TOKEN_PROGRAM_ID,
			})
			.rpc();
		console.log('Your transaction signature', tx);
	});
	it('Refund and close escrow!', async () => {
		const tx = await program.methods
			.refund()
			.accounts({
				maker: maker,
				mintA: mint,
				mintB: mint,
				tokenProgram: TOKEN_PROGRAM_ID,
			})
			.rpc();
		console.log('Your transaction signature', tx);
	});
});
