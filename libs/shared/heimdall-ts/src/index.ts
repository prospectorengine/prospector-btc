// INICIO DEL ARCHIVO [libs/shared/heimdall-ts/src/index.ts]
/**
 * =================================================================
 * APARATO: HEIMDALL PUBLIC INTERFACE (V36.0 - TRANSPORT EXPOSED)
 * CLASIFICACIÓN: EXPORT BARREL
 * RESPONSABILIDAD: PUNTO DE ENTRADA ÚNICO PARA CONSUMIDORES
 *
 * VISION HIPER-HOLÍSTICA:
 * Centraliza todas las exportaciones.
 * ✅ MEJORA: Se expone 'configureHeimdallUplink' para permitir la
 * inicialización del beacon remoto desde el Dashboard.
 * =================================================================
 */

import { SovereignLogger } from "./lib/logger";

// Exportación de tipos para consumidores
export { SovereignLogger } from "./lib/logger";
export { OperationSpan } from "./lib/tracer";
export type { LogSeverity, AuditMetadata } from "./lib/types";

// ✅ NUEVO: Exportación del configurador de transporte
export { configureHeimdallUplink } from "./lib/transports";

/**
 * Factoría estática para instanciar loggers con contexto.
 * @param context_name Identificador del módulo (ej: 'AuthHandler', 'Network')
 */
export const createLogger = (context_name: string) => new SovereignLogger(context_name);
// FIN DEL ARCHIVO [libs/shared/heimdall-ts/src/index.ts]
