/**
 * =================================================================
 * APARATO: IDENTITY GOVERNANCE HOOK (V2.2 - GOLD MASTER)
 * CLASIFICACIÓN: REACTIVE ADAPTER (ESTRATO L5)
 * RESPONSABILIDAD: GESTIÓN DE CICLO DE VIDA Y AUDITORÍA CRIPTOGRÁFICA
 * =================================================================
 */

import { useCallback } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import {
  adminApi,
  type Identity,
  type IdentityStatus,
  type EncryptedIdentityPayload
} from "@prospector/api-client";
import { VaultCryptoEngine } from "@prospector/crypto-vault";
import { IdentityHealthEngine, type IdentityHealthMetrics } from "@/lib/utils/identity-health";

/**
 * Interface del resultado de una auditoría profunda.
 */
export interface DeepAuditResult {
  identity_email: string;
  decrypted_cookies_count: number;
  health_metrics: IdentityHealthMetrics;
}

export function useIdentityGovernance() {
  const query_client = useQueryClient();
  const QUERY_KEY = ["identities-governance-inventory"];

  // 1. ADQUISICIÓN DE INVENTARIO (Polling Táctico cada 10s)
  const { data: identities, isLoading, isError } = useQuery({
    queryKey: QUERY_KEY,
    queryFn: () => adminApi.listIdentities(),
    refetchInterval: 10000,
  });

  // 2. MUTACIÓN: LIBERACIÓN FORZADA (Break Lease)
  const release_mutation = useMutation({
    mutationFn: (email: string) => adminApi.forceReleaseIdentity(email, "MANUAL_DASHBOARD_OVERRIDE"),
    onSuccess: (_, email) => {
      toast.success("LEASE_BROKEN", { description: `Identity [${email}] released to pool.` });
      query_client.invalidateQueries({ queryKey: QUERY_KEY });
    },
    onError: (err: Error) => toast.error("RELEASE_FAILED", { description: err.message })
  });

  // 3. MUTACIÓN: REPORTE DE MALFUNCIONAMIENTO (Kill-Switch)
  const malfunction_mutation = useMutation({
    mutationFn: ({ email, status }: { email: string; status: IdentityStatus }) =>
        adminApi.reportIdentityMalfunction(email, status),
    onSuccess: (_, variables) => {
      toast.warning("STATUS_DEGRADED", { description: `[${variables.email}] marked as ${variables.status.toUpperCase()}.` });
      query_client.invalidateQueries({ queryKey: QUERY_KEY });
    },
    onError: (err: Error) => toast.error("MALFUNCTION_REPORT_FAILED", { description: err.message })
  });

  // 4. MUTACIÓN: PURGA DEFINITIVA (Incineration)
  const purge_mutation = useMutation({
    mutationFn: (email: string) => adminApi.purgeIdentity(email, "MANUAL_DASHBOARD_PURGE"),
    onSuccess: (_, email) => {
      toast.warning("IDENTITY_INCINERATED", { description: `[${email}] permanently removed.` });
      query_client.invalidateQueries({ queryKey: QUERY_KEY });
    },
    onError: (err: Error) => toast.error("PURGE_FAILED", { description: err.message })
  });

  // 5. MOTOR DE AUDITORÍA CRIPTOGRÁFICA (Client-Side Decryption)
  const execute_deep_audit = useCallback(async (
    identity: Identity,
    master_key: string
  ): Promise<DeepAuditResult> => {
    try {
      let raw_cookies_json = identity.credentials_json;

      if (raw_cookies_json.includes("cipher_text_base64")) {
        const encrypted_payload = JSON.parse(raw_cookies_json) as EncryptedIdentityPayload;
        raw_cookies_json = await VaultCryptoEngine.decryptPortable(
          encrypted_payload,
          master_key,
          identity.email
        );
      }

      const parsed_cookies = JSON.parse(raw_cookies_json);
      if (!Array.isArray(parsed_cookies)) throw new Error("INVALID_COOKIE_STRUCTURE");

      const health_metrics = IdentityHealthEngine.analyze(parsed_cookies);

      return {
        identity_email: identity.email,
        decrypted_cookies_count: parsed_cookies.length,
        health_metrics
      };

    } catch (error) {
      const msg = error instanceof Error ? error.message : "DECRYPTION_FAILED";
      throw new Error(`AUDIT_FAULT: ${msg}`);
    }
  }, []);

  // ✅ ESTRATO DE ESTADO CONSOLIDADO (Resolución de regresión TS2339)
  const isProcessing = release_mutation.isPending || malfunction_mutation.isPending || purge_mutation.isPending;

  return {
    identities,
    isLoading,
    isError,
    actions: {
      forceRelease: release_mutation.mutate,
      reportMalfunction: (email: string, status: IdentityStatus) => malfunction_mutation.mutate({ email, status }),
      purge: purge_mutation.mutate,
      auditIdentity: execute_deep_audit
    },
    state: {
      isReleasing: release_mutation.isPending,
      isPurging: purge_mutation.isPending,
      isMalfunctioning: malfunction_mutation.isPending,
      isProcessing // Semáforo global unificado
    }
  };
}
