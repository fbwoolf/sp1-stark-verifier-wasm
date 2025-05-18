import init, { verify_stark } from './pkg/sp1_stark_verifier_wasm.js';

async function run() {
    await init();
    // console.log(greet());

    try {
        // Load proof and verifying key data from files
        const proofResponse = await fetch('./eth-assets/fibonacci-proof.bin');
        const vkResponse = await fetch('./eth-assets/fibonacci-vk.bin');
        if (!proofResponse.ok || !vkResponse.ok) {
            throw new Error('Failed to load proof.bin or vk.bin');
        }
        const proofData = new Uint8Array(await proofResponse.arrayBuffer());
        const vkData = new Uint8Array(await vkResponse.arrayBuffer());
        console.log('proof', proofData);
        console.log('vk', vkData);
        verify_stark(proofData, vkData);
        console.log("Verification successful!");
    } catch (e) {
        console.error("Verification failed:", e);
    }
}

run();
