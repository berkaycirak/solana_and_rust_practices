import wallet from './wallet/wba-wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from '@metaplex-foundation/umi';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';
import { readFile } from 'fs/promises';
import { createBundlrUploader } from '@metaplex-foundation/umi-uploader-bundlr';

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

const uploader = createBundlrUploader(umi);

(async () => {
  const file = 'rabbit6.png';
  const buffer = await readFile(file);

  try {
    const image = createGenericFile(buffer, 'image/png');
    const [imageURI] = await uploader.upload([image]);
    console.log('Image URI uploaded-------------->', imageURI);
    return imageURI;
  } catch (error) {
    console.log(error);
  }
})();
