import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Gdupgrader } from "../target/types/gdupgrader";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount} from "@solana/spl-token";

// consts
const MULTISIG_PDA_SEED = "multisig_pda_seed";
const GIGS_VAULT_PDA_SEED = "gigs_vault_pda_seed";
const PROPOSAL_PDA_SEED = "proposal_pda_seed";

// globals
let multisigPda;
let gigsVault;
let proposalPda;
let gigsMint;
let senderGigsAta;

// utils
function to_lamports(num_sol) {
    return Math.round(num_sol * 1e9);
}

const ProposalType = {
    UpgradeProgram: { upgradeProgram: {} },
    SetAuthority: { setAuthority: {} },
};

describe("gdupgrader", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Gdupgrader as Program<Gdupgrader>;

    it("Initialize", async () => {

        // init pdas
        let [_multisigPda, _b1] = await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from(anchor.utils.bytes.utf8.encode(MULTISIG_PDA_SEED))],
            program.programId
        );
        multisigPda = _multisigPda;

        let [_gigsVault, _b2] = await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from(anchor.utils.bytes.utf8.encode(GIGS_VAULT_PDA_SEED))],
            program.programId
        );
        gigsVault = _gigsVault;

        let [_proposalPda, _b3] = await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from(anchor.utils.bytes.utf8.encode(PROPOSAL_PDA_SEED))],
            program.programId
        );
        proposalPda = _proposalPda;

        // init gigs mint
        let owner1 = anchor.web3.Keypair.generate();
        await program.provider.connection.confirmTransaction(
            await program.provider.connection.requestAirdrop(owner1.publicKey, to_lamports(10)),
            "confirmed"
        );

        gigsMint = await createMint(
            program.provider.connection,
            owner1,
            owner1.publicKey,
            null,
            4,
        );

        senderGigsAta = await createAccount(
            program.provider.connection,
            owner1,
            gigsMint,
            program.provider.publicKey,
        );

        await mintTo(
            program.provider.connection,
            owner1,
            gigsMint,
            senderGigsAta,
            owner1,
            1500
        );


        let approval_threshold = new anchor.BN(1000);
        let proposal_minimum = new anchor.BN(500);

        const tx = await program.methods.initialize(approval_threshold, proposal_minimum)
            .accounts({
                signer: program.provider.publicKey,
                multisigPda: multisigPda,
                proposal: proposalPda,
                gigsMint: gigsMint,
                gigsVault: gigsVault,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .rpc();
        console.log("Your transaction signature", tx);
    });

    it("Propose", async () => {

        let ballot = anchor.web3.Keypair.generate();

        let proposal_type = ProposalType.SetAuthority;
        let target_buffer = program.provider.publicKey; // TODO get account buffer of current program...
        let source_buffer = program.provider.publicKey;
        let new_authority = program.provider.publicKey; // TODO change to owner 1?
        let amount = new anchor.BN(500);

        // @ts-ignore
        const tx = await program.methods.propose(proposal_type, target_buffer, source_buffer, new_authority, amount)
            .accounts({
                signer: program.provider.publicKey,
                proposal: proposalPda,
                ballot: ballot.publicKey,
                gigsMint: gigsMint,
                gigsVault: gigsVault,
                senderGigsAta: senderGigsAta,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([ballot])
            .rpc();
        console.log("Your transaction signature", tx);
    });

    it("Cast ballot", async () => {

        let ballot = anchor.web3.Keypair.generate();
        let proposal_id = new anchor.BN(1);
        let amount = new anchor.BN(500);

        // @ts-ignore
        const tx = await program.methods.castBallot(proposal_id, amount)
            .accounts({
                signer: program.provider.publicKey,
                ballot: ballot.publicKey,
                proposal: proposalPda,
                gigsMint: gigsMint,
                gigsVault: gigsVault,
                senderGigsAta: senderGigsAta,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([ballot])
            .rpc();
        console.log("Your transaction signature", tx);
    });

});
