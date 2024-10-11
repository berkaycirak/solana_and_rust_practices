import {
	clusterApiUrl,
	Connection,
	Keypair,
	PublicKey,
} from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbine3Prereq } from "./programs/turbin3_prereq";
import wallet from "./wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection(
	clusterApiUrl("devnet"),
	"confirmed"
);

// Github account name
const github = Buffer.from("berkaycirak", "utf8");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
	commitment: "confirmed",
});

// Create our program
const program: Program<Turbine3Prereq> = new Program(IDL, provider);

// Create the PDA for our enrollment account
const enrollment_seeds = [
	Buffer.from("prereq"),
	keypair.publicKey.toBuffer(),
];

const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
	enrollment_seeds,
	program.programId
);

// Execute our enrollment transaction
(async () => {
	try {
		const txhash = await program.methods
			.complete(github)
			.accounts({
				signer: keypair.publicKey,
			})
			.signers([keypair])
			.rpc();
		console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
