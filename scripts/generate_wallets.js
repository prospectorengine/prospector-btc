/**
 * =================================================================
 * APARATO: BITCOIN TEST VECTOR GENERATOR (V1.1 - FIXED)
 * RESPONSABILIDAD: Generar 33 carteras de prueba para inyección en CSV.
 * ESTRATEGIA: Brainwallet SHA256 -> Secp256k1 -> P2PKH Uncompressed.
 * =================================================================
 */

const crypto = require('crypto');
const fs = require('fs');
const bitcoin = require('bitcoinjs-lib');
const ecc = require('tiny-secp256k1');
const { ECPairFactory } = require('ecpair');

// 1. Inicialización correcta de ECPair (Cambio crítico v6+)
const ECPair = ECPairFactory(ecc);

// Inicializar motor elíptico para bitcoinjs-lib (necesario para otras funciones)
bitcoin.initEccLib(ecc);

const phrases = [
  "power", "the", "peter", "and", "money", "password", "12345678", "qwerty",
  "bitcoin", "satoshi", "god", "love", "freedom", "master", "secret", "hell",
  "heaven", "dragon", "warrior", "ninja", "shadow", "winter", "summer", "autumn",
  "spring", "coffee", "pizza", "beer", "matrix", "identity", "prospector",
  "hydra", "zero"
];

let reportOutput = "=== PROSPECTOR TEST WALLETS REPORT ===\n\n";
let csvLines = "";

phrases.forEach((phrase, index) => {
  // 2. Generar Private Key (SHA256 de la frase)
  const privateKeyBuffer = crypto.createHash('sha256').update(phrase).digest();
  const privateKeyHex = privateKeyBuffer.toString('hex');

  // 3. Generar WIF usando la nueva factoría ECPair
  // Nota: Usamos compressed: false para generar direcciones Legacy reales (empezadas en 1)
  const keyPair = ECPair.fromPrivateKey(privateKeyBuffer, { compressed: false });
  const wif = keyPair.toWIF();

  // 4. Generar Dirección Bitcoin (P2PKH Legacy)
  const { address } = bitcoin.payments.p2pkh({ pubkey: keyPair.publicKey });

  // 5. Acumular Reporte
  reportOutput += `ID: VEC-${(index + 1).toString().padStart(2, '0')}\n`;
  reportOutput += `SEMILLA:  ${phrase}\n`;
  reportOutput += `PRIV_HEX: ${privateKeyHex}\n`;
  reportOutput += `WIF:      ${wif}\n`;
  reportOutput += `ADDRESS:  ${address}\n`;
  reportOutput += `------------------------------------------------------------\n`;

  // 6. Acumular líneas para el CSV (address,balance)
  csvLines += `${address},10000000\n`;
});

// Escribir a archivo físico
fs.writeFileSync('wallets.txt', reportOutput + "\n=== BLOQUE PARA INYECTAR EN CSV ===\n\n" + csvLines);

console.log("\n✅ ARCHIVO 'wallets.txt' GENERADO CON ÉXITO.");
console.log("Copia los datos de 'wallets.txt' e inyecta el bloque final en tu CSV.");
