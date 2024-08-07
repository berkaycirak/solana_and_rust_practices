import wallet from './wallet/wba-wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
	createMetadataAccountV3,
	CreateMetadataAccountV3InstructionAccounts,
	CreateMetadataAccountV3InstructionArgs,
	DataV2Args,
} from '@metaplex-foundation/mpl-token-metadata';
import {
	createSignerFromKeypair,
	signerIdentity,
	publicKey,
} from '@metaplex-foundation/umi';
import bs58 from 'bs58';

// Define our Mint address
const mint = publicKey(
	'Ez8AuJTZFbjGW4VVBGGWKc4ut9gNCETkjAruWAnKAn6v'
);

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(
	new Uint8Array(wallet)
);
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
	try {
		// Start here
		let accounts: CreateMetadataAccountV3InstructionAccounts = {
			mint,
			mintAuthority: signer,
		};

		let data: DataV2Args = {
			name: 'CRAK ',
			symbol: 'CRK',
			sellerFeeBasisPoints: 100,
			uri: 'https://arweave.net/QHOSkv7oFRGUKAsCaLJ5jGgYGXvqTAo-2AyxAQvgZLg',
			collection: null,
			creators: null,
			uses: null,
		};

		let args: CreateMetadataAccountV3InstructionArgs = {
			data,
			collectionDetails: null,
			isMutable: true,
		};

		let tx = createMetadataAccountV3(umi, {
			...accounts,
			...args,
		});

		let result = await tx.sendAndConfirm(umi);
		console.log(bs58.encode(result.signature));
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
