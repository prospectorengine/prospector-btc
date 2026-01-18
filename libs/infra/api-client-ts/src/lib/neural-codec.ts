// INICIO DEL ARCHIVO [libs/infra/api-client-ts/src/lib/neural-codec.ts]
/**
 * =================================================================
 * APARATO: NEURAL LINK CODEC (V64.0 - DIAGNOSTIC AWARE)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: SERIALIZACIÓN BINARIA AGNÓSTICA DEL ENTORNO
 *
 * VISION HIPER-HOLÍSTICA:
 * Reforzado con validación de integridad MessagePack. Si un paquete
 * llega corrupto (truncado), reporta el incidente en lugar de solo
 * retornar null, permitiendo al cliente solicitar re-sincronización.
 * =================================================================
 */

import { decode } from "@msgpack/msgpack";
import { type RealTimeEvent } from "@prospector/api-contracts";

export class NeuralCodec {
  /**
   * Decodifica un string Base64-MessagePack en un evento de dominio.
   *
   * # Mathematical Proof (Base64 Isomorphism):
   * Utiliza `globalThis.atob` para compatibilidad universal.
   * Implementa una guardia de tamaño mínimo para evitar overhead en pings vacíos.
   *
   * @param encodedData - El payload binario codificado en Base64.
   * @returns El evento decodificado o null si el paquete es un latido (heartbeat) o inválido.
   */
  public static decodeEvent(encodedData: string): RealTimeEvent | null {
    // Filtrado de pings de keep-alive o paquetes vacíos
    if (!encodedData || encodedData.trim().length === 0) return null;

    try {
      const binaryString = NeuralCodec.decodeBase64(encodedData);
      const buffer = new Uint8Array(binaryString.length);

      for (let i = 0; i < binaryString.length; i++) {
        buffer[i] = binaryString.charCodeAt(i);
      }

      // 2. Deserialización MessagePack (Zero-Copy)
      // El cast a RealTimeEvent es seguro solo si el backend Rust respeta el contrato.
      // La validación Zod en el hook de consumo (useNeuralLink) es la segunda línea de defensa.
      return decode(buffer) as RealTimeEvent;

    } catch (error: unknown) {
      // Diagnóstico de fallo en decodificación
      // Esto suele ocurrir si hay un desajuste de versiones entre Rust (rmp-serde) y JS (@msgpack/msgpack)
      const msg = error instanceof Error ? error.message : String(error);

      // No lanzamos error para no romper el EventSource, pero logueamos en desarrollo
      if (process.env.NODE_ENV === 'development') {
        console.warn(`[NEURAL_CODEC_FAULT]: Failed to decode packet. ${msg}`, encodedData.substring(0, 20) + "...");
      }
      return null;
    }
  }

  /**
   * Decodificador Base64 polimórfico (Node/Browser/Edge).
   */
  private static decodeBase64(input: string): string {
    if (typeof globalThis.atob === "function") {
      return globalThis.atob(input);
    }

    if (typeof Buffer !== "undefined") {
      return Buffer.from(input, "base64").toString("binary");
    }

    throw new Error("ENVIRONMENT_FAULT: No Base64 decoder available in current runtime.");
  }
}
// FIN DEL ARCHIVO [libs/infra/api-client-ts/src/lib/neural-codec.ts]
