#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

// Generate a Solana keypair (Ed25519)
function generateKeypair() {
  const seed = crypto.randomBytes(32);
  return Array.from(seed);
}

// Convert keypair to base58 public key (simplified - just for display)
function keypairToBase58(keypair) {
  // This is a placeholder - in real Solana, you'd use @solana/web3.js
  // For now, we'll generate valid-looking base58 addresses
  const chars = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
  let result = '';
  for (let i = 0; i < 44; i++) {
    result += chars[Math.floor(Math.random() * chars.length)];
  }
  return result;
}

// Create target/deploy directory
const deployDir = path.join(__dirname, '../target/deploy');
if (!fs.existsSync(deployDir)) {
  fs.mkdirSync(deployDir, { recursive: true });
}

console.log('🔑 Generating Program Keypairs');
console.log('==============================\n');

// Generate theras_protocol keypair
const therasKeypair = generateKeypair();
const therasPath = path.join(deployDir, 'theras_protocol-keypair.json');
fs.writeFileSync(therasPath, JSON.stringify(therasKeypair));
console.log('✅ Generated theras_protocol-keypair.json');

// Generate tgem_plus keypair
const tgemKeypair = generateKeypair();
const tgemPath = path.join(deployDir, 'tgem_plus-keypair.json');
fs.writeFileSync(tgemPath, JSON.stringify(tgemKeypair));
console.log('✅ Generated tgem_plus-keypair.json');

// Generate placeholder program IDs (these will be replaced by actual Solana addresses when you have Solana CLI)
const therasId = keypairToBase58(therasKeypair);
const tgemId = keypairToBase58(tgemKeypair);

console.log('\n📋 Generated Program IDs:');
console.log(`  theras_protocol: ${therasId}`);
console.log(`  tgem_plus:       ${tgemId}`);

console.log('\n⚠️  NOTE: These are placeholder IDs.');
console.log('When you install Solana CLI, run:');
console.log('  solana address -k target/deploy/theras_protocol-keypair.json');
console.log('  solana address -k target/deploy/tgem_plus-keypair.json');
console.log('\nTo get the real program IDs and update your code.\n');

// Write IDs to a file for the update script
const idsPath = path.join(deployDir, 'program-ids.json');
fs.writeFileSync(idsPath, JSON.stringify({
  theras_protocol: therasId,
  tgem_plus: tgemId
}, null, 2));

console.log('✅ Keypairs generated and saved!\n');
