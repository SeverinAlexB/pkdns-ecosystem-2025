/**
 * Publish a DNS zone.
 * 
 * npm run start --file=1_publish_dns_zone
 */

import { Client, SignedPacket, Keypair } from '@synonymdev/pkarr';

async function main() {
    const client = new Client();
    const keypair = new Keypair();
    console.log("Created keypair", keypair.public_key_string());
    
    // Publish a packet
    const builder = SignedPacket.builder();
    // IP Address of the demo website
    builder.addARecord(".", "34.65.109.99", 300);
    console.log("Building packet");
    const packet = builder.buildAndSign(keypair);
    console.log("Publishing packet");
    await client.publish(packet);  
    console.log("Published packet!");
    
    // Resolve it again
    let resolved_packet = await client.resolve(keypair.public_key_string());
    if (!resolved_packet) {
        console.log("No packet found");
        return;
    }

    console.log("Resolved packet timestamp", resolved_packet.timestampMs);
    console.log("Record count", resolved_packet.recordCount);
    for (let record of resolved_packet.records) {
        console.log("- Record", record);
    }

    console.log(`Open http://${keypair.public_key_string()}/ to see the result.`);
};

try {
    main();
} catch (error) {
    console.error(error);
}
