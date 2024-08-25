import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { CustomBank } from '../target/types/custom_bank';
import { PublicKey } from '@solana/web3.js';

describe('custom_bank', () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.CustomBank as Program<CustomBank>;

	const token = new PublicKey(
		'6ioBBfscpe28T3JPzocBDKU7bPkq1DaUKNCC7GMCaXuJ'
	);

	it('Is initialized!', async () => {
		// Add your test here.
		const tx = await program.methods
			.createVault()
			.accounts({ token })
			.rpc();
		console.log('Your transaction signature', tx);
	});
});
