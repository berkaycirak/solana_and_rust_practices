import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { BasicCrud } from '../target/types/basic_crud';
import { Keypair } from '@solana/web3.js';

describe('basic_crud', () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.BasicCrud as Program<BasicCrud>;
	const wallet = Keypair.generate();

	// it('Is initialized!', async () => {
	// 	// Add your test here.
	// 	const tx = await program.methods.create('Berkay', 26).rpc();
	// 	console.log('Your transaction signature', tx);
	// });

	it('Read the data', async () => {
		const data = await program.account.newAccount.all();
		console.log(data);
	});

	it('Update Account', async () => {
		const tx = await program.methods.update('Berkay CRK', 27).rpc();
		console.log('Your transaction signature', tx);
	});

	it('Read new data', async () => {
		const data = await program.account.newAccount.all();
		console.log(data);
	});

	it('Delete Account', async () => {
		const tx = await program.methods.delete().rpc();
		console.log('Your transaction signature', tx);
	});

	it('Try reading the data after delete', async () => {
		const data = await program.account.newAccount.all();
		console.log(data);
	});
});
