/**
 * =================================================================
 * APARATO: SUPABASE STRATEGIC CLIENT (V26.5 - TypeScript Native)
 * CLASIFICACIÓN: INFRASTRUCTURE ADAPTER (ESTRATO L4)
 * RESPONSABILIDAD: GESTIÓN DE ENLACES CON EL MOTOR ESTRATÉGICO
 *
 * NOTA TÁCTICA: Este archivo DEBE tener extensión .ts para soportar
 * anotaciones de tipo y contratos de interfaz.
 * =================================================================
 */

import { createBrowserClient } from '@supabase/ssr';
import { type SupabaseClient } from '@supabase/supabase-js';
import { strategicCensus } from "./lib/census";

// 1. ADQUISICIÓN DE CREDENCIALES (Runtime Awareness)
const SUPABASE_URL: string = process.env.NEXT_PUBLIC_SUPABASE_URL || "";
const SUPABASE_ANON_KEY: string = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY || "";

/**
 * Cliente de Motor B: Repositorio Estratégico de la Tesis.
 * Implementa el modo SSR compatible con Next.js 16.
 */
export const supabase: SupabaseClient = createBrowserClient(
  SUPABASE_URL,
  SUPABASE_ANON_KEY
);

export { strategicCensus };

/**
 * Adaptador de archivo histórico de misiones certificadas.
 */
export const strategicArchive = {
  /**
   * Recupera el histórico de auditoría desde el Motor Estratégico.
   * @param limit_records - Cantidad de registros a extraer (Default: 20).
   */
  getHistory: async (limit_records: number = 20) => {
    const { data, error } = await supabase
      .from("archived_audit_reports")
      .select("*")
      .order("created_at", { ascending: false })
      .limit(limit_records);

    if (error) {
      throw new Error(`STRATEGIC_UPLINK_FAULT: ${error.message}`);
    }

    return data;
  },

  /**
   * Recupera métricas agregadas de esfuerzo computacional global.
   */
  getGlobalMetrics: async () => {
    const { data, error } = await supabase
      .from("system_integrity_reports")
      .select("*")
      .order("detected_at_timestamp", { ascending: false })
      .limit(1)
      .single();

    if (error && error.code !== 'PGRST116') {
      throw new Error(`METRICS_UNREACHABLE: ${error.message}`);
    }

    return data;
  }
};
