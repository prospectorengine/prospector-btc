// INICIO DEL ARCHIVO [libs/infra/api-client-ts/src/lib/client.ts]
/**
 * =================================================================
 * APARATO: RESILIENT API CLIENT (V17.0 - GRAPHQL ENABLED)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L4)
 * RESPONSABILIDAD: GESTIÓN DE CANALES REST Y GRAPHQL
 *
 * VISION HIPER-HOLÍSTICA:
 * Integra el método 'graphql' con unwrapping automático de 'data'.
 * Define y exporta 'neuralOracle' como la fachada especializada.
 * =================================================================
 */

import axios, {
  type AxiosInstance,
  type AxiosRequestConfig,
  type InternalAxiosRequestConfig,
  type AxiosResponse
} from 'axios';
import { createLogger } from '@prospector/heimdall-ts';

const logger = createLogger("API_Client");

interface GraphQLResponse<T> {
  data: T;
  errors?: Array<{ message: string; locations?: unknown[]; path?: string[] }>;
}

export class ResilientApiClient {
  private network_instance: AxiosInstance;

  /**
   * @param target_layer 'tactical' (Rust/L3) o 'local' (Next.js/L4)
   */
  constructor(target_layer: 'tactical' | 'local' = 'tactical') {
    const is_browser = typeof window !== 'undefined';

    // CANAL TÁCTICO (RUST): Usa /api/v1 para activar el Proxy
    // CANAL LOCAL (NEXT): Usa /api para golpear los Route Handlers directamente
    const base_path = target_layer === 'tactical' ? '/api/v1' : '/api';

    const gateway_base_url = is_browser
      ? base_path
      : (process.env.NEXT_PUBLIC_SITE_URL || 'http://localhost:3000') + base_path;

    this.network_instance = axios.create({
      baseURL: gateway_base_url,
      timeout: 25000,
      headers: { 'Content-Type': 'application/json' },
    });

    this.configure_interceptors();
  }

  private configure_interceptors(): void {
    this.network_instance.interceptors.request.use((config: InternalAxiosRequestConfig) => {
      const is_browser = typeof window !== 'undefined';
      const session_token = is_browser
        ? sessionStorage.getItem('ADMIN_SESSION_TOKEN')
        : process.env.WORKER_AUTH_TOKEN;

      if (session_token && config.headers) {
        config.headers.Authorization = `Bearer ${session_token}`;
      }
      return config;
    });

    this.network_instance.interceptors.response.use(
      (response: AxiosResponse) => response,
      (network_error) => {
        const url = network_error.config?.url;
        const status = network_error.response?.status || "TIMEOUT";
        // Silenciar logs para errores controlados (ej. sondas de diagnóstico)
        if (status !== 404) {
             logger.error(`UPLINK_FAULT: ${url} [${status}]`);
        }
        return Promise.reject(network_error);
      }
    );
  }

  public async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.network_instance.get<T>(url, config);
    return response.data;
  }

  public async post<T>(url: string, payload?: unknown, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.network_instance.post<T>(url, payload, config);
    return response.data;
  }

  /**
   * Ejecuta una consulta contra el Neural Data Gateway.
   *
   * # Logic:
   * Envuelve la query en un payload JSON estándar.
   * Desempaqueta 'data' y lanza error si 'errors' está presente.
   */
  public async graphql<T>(query: string, variables?: Record<string, unknown>): Promise<T> {
    const payload = { query, variables };
    // El endpoint es relativo a la base (/api/v1 + /graphql)
    const response = await this.post<GraphQLResponse<T>>('/graphql', payload);

    if (response.errors && response.errors.length > 0) {
      const primary_error = response.errors[0].message;
      logger.warn(`GRAPHQL_REJECTION: ${primary_error}`);
      throw new Error(`NEURAL_QUERY_FAILED: ${primary_error}`);
    }

    return response.data;
  }
}

// INSTANCIA PRINCIPAL (Apunta al Orquestador Rust)
export const apiClient = new ResilientApiClient('tactical');

// INSTANCIA LOCAL (Apunta a Next.js Serverless Functions)
export const nextApiClient = new ResilientApiClient('local');

/**
 * FACHADA: NEURAL ORACLE
 * Interfaz especializada para consumo de datos complejos en el Dashboard.
 */
export const neuralOracle = {
  /**
   * Envía una consulta al grafo de conocimiento.
   * @param query String de consulta GQL.
   * @param variables Variables opcionales.
   */
  query: async <T>(query: string, variables?: Record<string, unknown>): Promise<T> => {
    return await apiClient.graphql<T>(query, variables);
  }
};
// FIN DEL ARCHIVO [libs/infra/api-client-ts/src/lib/client.ts]
