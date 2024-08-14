import wallet from './wallet/wba-wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
	createMetadataAccountV3,
	CreateMetadataAccountV3InstructionAccounts,
	CreateMetadataAccountV3InstructionArgs,
	DataV2Args,
	mplTokenMetadata,
} from '@metaplex-foundation/mpl-token-metadata';
import {
	createSignerFromKeypair,
	signerIdentity,
	publicKey,
} from '@metaplex-foundation/umi';
import bs58 from 'bs58';
import { Connection } from '@solana/web3.js';

// Define our Mint address
const mint = publicKey(
	'DxhNFBqUkQr2DJ964aovMJBFwCFoe1vr8WDzjEyrUy7H'
);
const walletPk = [
	167, 43, 92, 26, 154, 54, 14, 155, 110, 56, 184, 186, 226, 102, 149,
	25, 162, 176, 188, 114, 62, 177, 55, 76, 12, 132, 137, 181, 114,
	204, 171, 203, 108, 121, 143, 184, 24, 20, 65, 111, 39, 230, 18, 62,
	125, 51, 12, 30, 200, 115, 108, 197, 244, 12, 175, 44, 115, 14, 80,
	243, 168, 8, 36, 160,
];

// Create a UMI connection
const umi = createUmi(new Connection('http://127.0.0.1:8899'));
const keypair = umi.eddsa.createKeypairFromSecretKey(
	new Uint8Array(walletPk)
);

const signer = createSignerFromKeypair(umi, keypair);
umi.use(mplTokenMetadata());
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
