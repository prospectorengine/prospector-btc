/**
 * =================================================================
 * APARATO: ADMIN API ADAPTER (V40.0 - SOBERANO)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L4)
 * RESPONSABILIDAD: ORQUESTACIÓN DE MANDO Y GOBERNANZA DE IDENTIDAD
 *
 * VISION HIPER-HOLÍSTICA:
 * Actúa como el puente de mando entre el Dashboard (L5) y el
 * Orquestador Rust (L3). Implementa el protocolo IGFS (Identity
 * Governance & Forensic Suite) con trazabilidad total.
 * =================================================================
 */

import { apiClient } from "./client";
import {
  type Identity,
  type IdentityPayload,
  type IdentityGovernancePayload,
  type IdentityStatus,
  IdentitySchema
} from "@prospector/api-contracts";

/**
 * Adaptador táctico para la gestión administrativa del sistema Prospector.
 */
export const adminApi = {
  /**
   * Transmite una nueva identidad cifrada (Zero-Knowledge) a la bóveda.
   *
   * @param payload - Datos de identidad incluyendo el material de cookies cifrado en cliente.
   * @returns Una promesa que se resuelve tras la cristalización en el Ledger Táctico.
   */
  uploadIdentity: async (payload: IdentityPayload): Promise<void> => {
    return await apiClient.post("/admin/identities", payload);
  },

  /**
   * Recupera el inventario íntegro de identidades del pool de minería.
   *
   * # Errors
   * Dispara una alerta de esquema si los datos de Turso no satisfacen el contrato L2.
   *
   * @returns Colección de identidades validadas por Zod.
   */
  listIdentities: async (): Promise<Identity[]> => {
    const raw_response = await apiClient.get<unknown[]>("/admin/identities");

    // Validación de integridad bit a bit contra el esquema de dominio
    return raw_response.map((item, index) => {
      try {
        return IdentitySchema.parse(item);
      } catch (validation_error) {
        throw new Error(`CONTRACT_MISMATCH: Identity at index [${index}] is corrupt. ${validation_error}`);
      }
    });
  },

  /**
   * Ejecuta una auditoría rápida de la capacidad de cómputo basada en identidades disponibles.
   *
   * @returns Resumen de nodos activos y contratos de arrendamiento vigentes.
   */
  checkIdentityStatus: async (): Promise<{ nodeCount: number; activeLeases: number }> => {
    const identities_collection = await adminApi.listIdentities();
    return {
      nodeCount: identities_collection.length,
      activeLeases: identities_collection.filter(i => i.usage_count > 0).length,
    };
  },

  // --- ESTRATO DE GOBERNANZA IGFS (IDENTITY GOVERNANCE) ---

  /**
   * Rompe el bloqueo de arrendamiento de una identidad.
   * Vital para recuperar hilos de workers que han muerto sin cerrar sesión.
   *
   * @param email - Identificador de la cuenta objetivo.
   * @param reason - Justificación forense para la liberación manual.
   */
  forceReleaseIdentity: async (email: string, reason: string = "MANUAL_INTERVENTION"): Promise<void> => {
    const payload: IdentityGovernancePayload = { email, reason };
    return await apiClient.post("/admin/identities/release", payload);
  },

  /**
   * Degrada el estado de una identidad tras detectar fallos de red o baneo.
   * Sincronizado con el protocolo de enfriamiento (Cooldown) de 24h en Rust.
   *
   * @param email - Cuenta afectada.
   * @param status - Nuevo estado: 'ratelimited' o 'revoked'.
   */
  reportIdentityMalfunction: async (email: string, status: IdentityStatus): Promise<void> => {
    return await apiClient.post("/admin/identities/revoke", { email, status });
  },

  /**
   * Ejecuta la incineración física de una identidad en el Ledger.
   * Acción irreversible diseñada para la depuración de rastro y privacidad.
   *
   * @param email - Identificador de la cuenta a eliminar.
   * @param reason - Motivo de la purga (ej: 'DATA_SANITIZATION').
   */
  purgeIdentity: async (email: string, reason: string = "DATA_SANITIZATION"): Promise<void> => {
    const payload: IdentityGovernancePayload = { email, reason };
    return await apiClient.post("/admin/identities/purge", payload);
  }
};
