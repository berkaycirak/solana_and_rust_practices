import wallet from './wallet/wba-wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from '@metaplex-foundation/umi';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';
import { createBundlrUploader } from '@metaplex-foundation/umi-uploader-bundlr';

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

const uploader = createBundlrUploader(umi);
(async () => {
  try {
    const image =
      'https://arweave.net/n40G0W-FjZJCOwSpvUBhjQVR3AhetwTsZlZ-pFjfre4';
    const metadata = {
      name: 'Rabbido',
      symbol: 'RBD',
      description: 'That is a warrior rabiddo',
      image,
      attributes: [{ trait_type: 'rarirty', value: 'legendary' }],
      properties: {
        files: [
          {
            type: 'image/png',
            uri: image,
          },
        ],
      },
      creators: [],
    };

    const myUri = await uploader.uploadJson(metadata);
    console.log('Your image URI: ', myUri);
  } catch (error) {
    console.log('Oops.. Something went wrong', error);
  }
})();
