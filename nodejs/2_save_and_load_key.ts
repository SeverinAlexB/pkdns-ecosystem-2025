/**
 * Generate, save, and load a keypair to a file in the standard hex format.
 * 
 * npm run start --file=2_save_and_load_key
 */

import { Keypair } from '@synonymdev/pkarr';
import * as fs from 'fs';

const KEY_FILE = "secret.hex";


/**
 * Save the keypair secret as hex to a file.
 */
function saveKey(keypair: Keypair) {
    const hex = Buffer.from(keypair.secret_key_bytes()).toString('hex');
    fs.writeFileSync(KEY_FILE, hex);
}

/**
 * Load the keypair secret from a hex file.
 */
function loadKeypair(): Keypair {
    if (!fs.existsSync(KEY_FILE)) {
        throw new Error(`Key file ${KEY_FILE} not found`);
    }
    
    const hex = fs.readFileSync(KEY_FILE, 'utf8').trim();
    const secretKeyBytes = Buffer.from(hex, 'hex');
    return Keypair.from_secret_key(secretKeyBytes);
}


async function main() {
    const keypair = new Keypair();
    console.log("Created keypair", keypair.public_key_string());
    saveKey(keypair);
    console.log("Saved key to", KEY_FILE);
    
    // Load the keypair back
    const loadedKeypair = loadKeypair();
    console.log("Loaded keypair", loadedKeypair.public_key_string());
};

try {
    main();
} catch (error) {
    console.error(error);
}
