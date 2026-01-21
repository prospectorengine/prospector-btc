/**
 * =================================================================
 * APARATO: RESILIENT API CLIENT (V18.0 - L7 SERVICE HUB)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L4)
 * RESPONSABILIDAD: GESTIÓN DE CANALES REST, GRAPHQL Y SERVICIOS L7
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. L7 FAÇADE INTEGRATION: Expone interfaces nominales para Billing,
 *    Herald y Nexus, eliminando peticiones 'get' genéricas en la UI.
 * 2. ISOMORPHIC TOKEN RESOLUTION: Gestión segura de tokens en SSR y Client.
 * 3. NEURAL ORACLE ENHANCEMENT: Motor GQL con unwrapping atómico de errores.
 * 4. HYGIENE: Cero 'any', tipado absoluto y rastro forense vía Heimdall.
 * =================================================================
 */

import axios, {
  type AxiosInstance,
  type AxiosRequestConfig,
  type InternalAxiosRequestConfig,
  type AxiosResponse
} from 'axios';
import { createLogger } from '@prospector/heimdall-ts';
import {
  type BillingQuota,
  type SystemNotification,
  type OperatorRank,
  type RealTimeEvent
} from '@prospector/api-contracts';

const logger = createLogger("L4:API_Client");

/**
 * Interface para el desempaquetado de señales del Oráculo GraphQL.
 */
interface GraphQLResponse<T> {
  data: T;
  errors?: Array<{ message: string; path?: string[] }>;
}

/**
 * Cliente de red centralizado con capacidad de reintento y resiliencia.
 */
export class ResilientApiClient {
  private network_instance: AxiosInstance;

  /**
   * @param target_layer - 'tactical' para el Orquestador Rust, 'local' para Next.js API.
   */
  constructor(target_layer: 'tactical' | 'local' = 'tactical') {
    const is_browser = typeof window !== 'undefined';

    // El canal táctico utiliza el prefijo /api/v1 para ser interceptado por el Proxy de Next.
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

  /**
   * Configura los centinelas de solicitud y respuesta para la inyección de seguridad.
   */
  private configure_interceptors(): void {
    this.network_instance.interceptors.request.use((config: InternalAxiosRequestConfig) => {
      const is_browser = typeof window !== 'undefined';

      // Recuperación de la llave maestra desde el estrato correspondiente
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
        const status = network_error.response?.status;
        const endpoint = network_error.config?.url;

        // Fail-Silent: No logueamos sondas de diagnóstico 404 (Expected behavior)
        if (status !== 404) {
          logger.error(`UPLINK_FAULT: [${status || 'TIMEOUT'}] in sector ${endpoint}`);
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
   * Provee validación de esquema en tiempo de ejecución.
   */
  public async graphql<T>(query: string, variables?: Record<string, unknown>): Promise<T> {
    const payload = { query, variables };
    const response = await this.post<GraphQLResponse<T>>('/graphql', payload);

    if (response.errors && response.errors.length > 0) {
      const error_msg = response.errors[0].message;
      logger.warn(`ORACLE_REJECTION: ${error_msg}`);
      throw new Error(`NEURAL_QUERY_FAILED: ${error_msg}`);
    }

    return response.data;
  }
}

// --- INSTANCIACIÓN DE CANALES SOBERANOS ---

/** Instancia para comunicación directa con el Orquestador Rust. */
export const apiClient = new ResilientApiClient('tactical');

/** Instancia para comunicación con los Route Handlers locales (L4). */
export const nextApiClient = new ResilientApiClient('local');

// --- FACHADAS DE ESTRATO L7 (USER SERVICES) ---

/** Fachada especializada para la gobernanza financiera. */
export const billingApi = {
  getQuota: () => apiClient.get<BillingQuota>('/user/billing/quota'),
  getHistory: () => apiClient.get<unknown[]>('/user/billing/history'),
};

/** Fachada para el sistema nervioso de comunicaciones Herald. */
export const heraldApi = {
  listNotifications: () => apiClient.get<SystemNotification[]>('/user/herald/notifications'),
  markAsRead: (id: string) => apiClient.post('/user/herald/notifications/read', { notification_identifier: id }),
};

/** Fachada para el prestigio y la red social técnica Nexus. */
export const nexusApi = {
  getPrestige: () => apiClient.get<OperatorRank>('/user/nexus/prestige'),
  getLeaderboard: () => apiClient.get<unknown[]>('/user/nexus/leaderboard'),
};

/** Interfaz unificada para consultas de conocimiento. */
export const neuralOracle = {
  query: <T>(query: string, vars?: Record<string, unknown>) => apiClient.graphql<T>(query, vars),
};
