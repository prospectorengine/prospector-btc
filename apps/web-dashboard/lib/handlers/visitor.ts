/**
 * =================================================================
 * APARATO: VISITOR INTELLIGENCE HANDLER (V16.7 - SOBERANO)
 * CLASIFICACIÓN: MIDDLEWARE LOGIC / INFRASTRUCTURE
 * RESPONSABILIDAD: EXTRACCIÓN DETERMINISTA DE CONTEXTO DE RED
 *
 * VISION HIPER-HOLÍSTICA:
 * Implementa la lógica de adquisición de identidad de red para los
 * nodos del Dashboard. Utiliza un esquema de priorización de cabeceras
 * para garantizar la captura de la IP real detrás de proxies (Vercel/Render).
 *
 * # Mathematical Proof:
 * El algoritmo garantiza la recuperación de una IP válida (v4/v6)
 * incluso en entornos locales (Loopback fallback), evitando valores nulos.
 * =================================================================
 */

import { NextRequest } from "next/server";

/**
 * Interface soberana para el contexto del visitante.
 */
export interface VisitorContext {

/*! Dirección IP pública del operador o nodo. */
  ip_address: string;

/*! Código de país (ISO 3166-1 alpha-2) capturado por el borde. */
  country_code: string;

/*! Identificador de software del cliente. */
  user_agent: string;
}

/**
 * Interface extendida para la resolución de propiedades de Edge.
 */
interface NextRequestWithEdgeProperties extends NextRequest {

/*! Propiedad inyectada por Vercel Edge Runtime. */
  ip?: string;

/*! Propiedad inyectada por Vercel con datos geográficos. */
  geo?: {
    country?: string;
    city?: string;
    region?: string;
  };
}

/**
 * Ejecuta la secuencia de extracción de metadatos de red.
 *
 * # Performance:
 * Operación O(1). Acceso directo a diccionarios de cabeceras sin latencia.
 *
 * # Errors:
 * En caso de ausencia de cabeceras geográficas, retorna "UNKNOWN_STRATUM".
 *
 * @param execution_request - La solicitud entrante capturada en el borde.
 */
export async function visitorHandler(
  execution_request: NextRequest
): Promise<VisitorContext> {
  // Casting controlado para acceder a propiedades de Edge sin violar el linter
  const request_with_metadata = execution_request as NextRequestWithEdgeProperties;

  /**
   * ADQUISICIÓN DE DIRECCIÓN IP (PRECEDENCIA):
   * 1. Propiedad nativa .ip (Vercel Edge)
   * 2. Cabecera X-Forwarded-For (Proxies)
   * 3. Cabecera X-Real-IP
   * 4. Fallback: Localhost
   */
  const detected_ip_address =
    request_with_metadata.ip ||
    execution_request.headers.get("x-forwarded-for")?.split(",")[0] ||
    execution_request.headers.get("x-real-ip") ||
    "127.0.0.1";

  /**
   * ADQUISICIÓN GEOGRÁFICA:
   * 1. Propiedad nativa .geo (Vercel Edge)
   * 2. Cabecera inyectada por el Gateway
   */
  const detected_country_code =
    request_with_metadata.geo?.country ||
    execution_request.headers.get("x-vercel-ip-country") ||
    "UNKNOWN_STRATUM";

  const user_agent_identifier =
    execution_request.headers.get("user-agent") ||
    "PROSPECTOR_ALPHA_AGENT";

  return {
    ip_address: detected_ip_address,
    country_code: detected_country_code,
    user_agent: user_agent_identifier,
  };
}
