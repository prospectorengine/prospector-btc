// INICIO DEL ARCHIVO [libs/core/client-vault/src/lib/aes-gcm.ts]
/**
 * =====================================================================
 * APARATO: DETERMINISTIC VAULT ENGINE (V59.0 - CONTRACT COMPLIANT)
 * CLASIFICACIÓN: CORE SECURITY (ESTRATO L1)
 * RESPONSABILIDAD: CIFRADO AES-256-GCM SOBERANO (BROWSER NATIVE)
 * =====================================================================
 */

// Definición local para independencia, pero debe coincidir estructuralmente con el contrato.
export interface EncryptedVaultPayload {
  cipher_text_base64: string;
  initialization_vector_base64: string;
  salt_base64: string;
}

export class VaultCryptoEngine {
  private static readonly CRYPTO_ALGORITHM = "AES-GCM";
  private static readonly DERIVATION_ALGORITHM = "PBKDF2";
  private static readonly HASH_FUNCTION = "SHA-256";
  private static readonly KEY_LENGTH_BITS = 256;
  private static readonly PBKDF2_ITERATIONS = 150_000; // Sincronizado con Rust

  private static normalizeToBufferSource(data: Uint8Array | ArrayBuffer): BufferSource {
    return data as BufferSource;
  }

  /**
   * Cifra un texto plano utilizando una frase maestra derivada.
   * Genera un payload compatible con `EncryptedIdentityPayload`.
   *
   * # Performance
   * Utiliza `window.crypto.subtle` para aceleración por hardware en el navegador.
   */
  public static async encryptPortable(
    plainText: string,
    masterPassphrase: string,
    operatorEmail: string
  ): Promise<EncryptedVaultPayload> {
    if (typeof window === "undefined" || !window.crypto) {
      throw new Error("ENVIRONMENT_FAULT: WebCrypto API unavailable. Are you in a secure context (HTTPS)?");
    }

    const textEncoder = new TextEncoder();

    // Sal determinista vinculada al email del operador para evitar rainbow tables genéricas
    const saltMaterial = `prospector_strata_v1_${operatorEmail.toLowerCase()}`;
    const saltBuffer = textEncoder.encode(saltMaterial);

    // IV aleatorio de 12 bytes (96 bits) estándar para GCM
    const initializationVector = window.crypto.getRandomValues(new Uint8Array(12));

    // Derivación de llave simétrica
    const derivedKey = await this.deriveSovereignKey(masterPassphrase, saltBuffer);

    const encodedPlainText = textEncoder.encode(plainText);

    const encryptedData = await window.crypto.subtle.encrypt(
      {
        name: this.CRYPTO_ALGORITHM,
        iv: this.normalizeToBufferSource(initializationVector),
      },
      derivedKey,
      this.normalizeToBufferSource(encodedPlainText)
    );

    return {
      cipher_text_base64: this.bufferToBase64(encryptedData),
      initialization_vector_base64: this.bufferToBase64(initializationVector.buffer),
      salt_base64: this.bufferToBase64(saltBuffer.buffer),
    };
  }

  public static async decryptPortable(
    payload: EncryptedVaultPayload,
    masterPassphrase: string,
    operatorEmail: string
  ): Promise<string> {
    if (typeof window === "undefined" || !window.crypto) {
      throw new Error("ENVIRONMENT_FAULT: WebCrypto API unavailable.");
    }

    const textDecoder = new TextDecoder();
    const textEncoder = new TextEncoder();

    const saltMaterial = `prospector_strata_v1_${operatorEmail.toLowerCase()}`;
    const saltBuffer = textEncoder.encode(saltMaterial);

    const ivBuffer = this.base64ToArrayBuffer(payload.initialization_vector_base64);
    const cipherBuffer = this.base64ToArrayBuffer(payload.cipher_text_base64);

    const derivedKey = await this.deriveSovereignKey(masterPassphrase, saltBuffer);

    try {
      const decryptedData = await window.crypto.subtle.decrypt(
        {
          name: this.CRYPTO_ALGORITHM,
          iv: this.normalizeToBufferSource(new Uint8Array(ivBuffer)),
        },
        derivedKey,
        this.normalizeToBufferSource(new Uint8Array(cipherBuffer))
      );

      return textDecoder.decode(decryptedData);
    } catch (error) {
      // Fallo de autenticación (Tag mismatch) o llave incorrecta
      throw new Error("VAULT_ACCESS_DENIED: Decryption failed. Verify Master Key integrity.");
    }
  }

  private static async deriveSovereignKey(
    passphrase: string,
    salt: Uint8Array
  ): Promise<CryptoKey> {
    const textEncoder = new TextEncoder();

    const keyMaterial = await window.crypto.subtle.importKey(
      "raw",
      this.normalizeToBufferSource(textEncoder.encode(passphrase)),
      { name: this.DERIVATION_ALGORITHM },
      false,
      ["deriveKey"]
    );

    return window.crypto.subtle.deriveKey(
      {
        name: this.DERIVATION_ALGORITHM,
        salt: this.normalizeToBufferSource(salt),
        iterations: this.PBKDF2_ITERATIONS,
        hash: this.HASH_FUNCTION,
      },
      keyMaterial,
      { name: this.CRYPTO_ALGORITHM, length: this.KEY_LENGTH_BITS },
      false,
      ["encrypt", "decrypt"]
    );
  }

  private static bufferToBase64(buffer: ArrayBuffer | ArrayBufferView | ArrayBufferLike): string {
    const bytes = new Uint8Array(buffer as ArrayBuffer);
    let binaryString = "";
    for (let i = 0; i < bytes.byteLength; i++) {
      binaryString += String.fromCharCode(bytes[i]);
    }
    return window.btoa(binaryString);
  }

  private static base64ToArrayBuffer(base64String: string): ArrayBuffer {
    const binaryString = window.atob(base64String);
    const buffer = new ArrayBuffer(binaryString.length);
    const byteArray = new Uint8Array(buffer);
    for (let i = 0; i < binaryString.length; i++) {
      byteArray[i] = binaryString.charCodeAt(i);
    }
    return buffer;
  }
}
// FIN DEL ARCHIVO [libs/core/client-vault/src/lib/aes-gcm.ts]
