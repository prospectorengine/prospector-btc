/**
 * =================================================================
 * APARATO: RESILIENT API CLIENT (V18.8 - PRODUCTION HARDENED)
 * CLASIFICACIÓN: INFRASTRUCTURE LAYER (ESTRATO L4)
 * RESPONSABILIDAD: GESTIÓN DE CANALES REST, GRAPHQL Y SERVICIOS L7
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. COLD-START RESILIENCE: Implementa interceptores de reintento para
 *    mitigar la hibernación de la nueva cuenta de Render.
 * 2. DISTRIBUTED TRACING: Inyecta un 'X-Trace-Id' unívoco por ráfaga
 *    para correlación bit-perfecta en el Motor C (MongoDB).
 * 3. ISOMORPHIC AUTHORITY: Resolución de tokens unificada para SSR
 *    (Vercel Edge) y CSR (Navegador).
 * 4. ZERO REGRESSIONS: Mantiene paridad total con las fachadas L7
 *    (Billing, Herald, Nexus) y el motor Neural Oracle.
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
} from '@prospector/api-contracts';

const logger = createLogger("L4:Resilient_Uplink");

/**
 * Interface para el desempaquetado de señales del Oráculo GraphQL.
 */
interface GraphQLResponse<T> {
  data: T;
  errors?: Array<{ message: string; path?: string[] }>;
}

/**
 * Cliente de red centralizado con blindaje contra latencia de nube.
 */
export class ResilientApiClient {
  private network_instance: AxiosInstance;

  /**
   * @param target_layer - 'tactical' para el Orquestador Rust, 'local' para Next.js API.
   */
  constructor(target_layer: 'tactical' | 'local' = 'tactical') {
    const is_browser = typeof window !== 'undefined';

    // El canal táctico utiliza el rewrite local /api/v1 para aprovechar el cache del borde
    const base_path = target_layer === 'tactical' ? '/api/v1' : '/api';

    const gateway_base_url = is_browser
      ? base_path
      : (process.env.NEXT_PUBLIC_SITE_URL || 'http://localhost:3000') + base_path;

    this.network_instance = axios.create({
      baseURL: gateway_base_url,
      timeout: 30000, // Elevado a 30s para absorber el bootstrapping del orquestador
      headers: {
        'Content-Type': 'application/json',
        'X-Prospector-Layer': 'L4_INFRA'
      },
    });

    this.configure_interceptors();
  }

  /**
   * Configura los centinelas de seguridad y observabilidad distribuida.
   */
  private configure_interceptors(): void {
    this.network_instance.interceptors.request.use((config: InternalAxiosRequestConfig) => {
      const is_browser = typeof window !== 'undefined';

      // 1. INYECCIÓN DE IDENTIDAD SOBERANA
      const session_token = is_browser
        ? sessionStorage.getItem('ADMIN_SESSION_TOKEN')
        : process.env.WORKER_AUTH_TOKEN;

      if (session_token && config.headers) {
        config.headers.Authorization = `Bearer ${session_token}`;
      }

      // 2. GENERACIÓN DE RASTRO FORENSE (Traceability)
      if (config.headers) {
        config.headers['X-Trace-Id'] = crypto.randomUUID();
      }

      return config;
    });

    this.network_instance.interceptors.response.use(
      (response: AxiosResponse) => response,
      async (network_error) => {
        const { config, response } = network_error;
        const status = response?.status;

        // PROTOCOLO DE REANIMACIÓN (Retry logic para Cold Starts)
        // Reintentamos una vez si recibimos 503 (Service Unavailable) o Timeout
        if ((status === 503 || network_error.code === 'ECONNABORTED') && !config._isRetry) {
          config._isRetry = true;
          logger.warn(`🚀 [REANIMATION]: Service hibernating. Attempting second pulse...`);
          return this.network_instance(config);
        }

        // Fail-Silent para diagnósticos 404, error real para el resto
        if (status !== 404) {
          logger.error(`UPLINK_FAULT: sector=[${config.url}] status=[${status || 'TIMEOUT'}]`);
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
   * Ejecuta una consulta contra el Neural Data Gateway (GraphQL).
   * Implementa desensamblado atómico de errores del oráculo.
   */
  public async graphql<T>(query: string, variables?: Record<string, unknown>): Promise<T> {
    const payload = { query, variables };
    const response = await this.post<GraphQLResponse<T>>('/graphql', payload);

    if (response.errors && response.errors.length > 0) {
      const error_artifact = response.errors[0];
      logger.warn(`ORACLE_REJECTION: ${error_artifact.message}`, { path: error_artifact.path });
      throw new Error(`NEURAL_QUERY_FAILED: ${error_artifact.message}`);
    }

    return response.data;
  }
}

// --- INSTANCIACIÓN DE CANALES SOBERANOS ---

/** Instancia principal conectada al Orquestador Rust. */
export const apiClient = new ResilientApiClient('tactical');

/** Instancia conectada a los Route Handlers de Next.js. */
export const nextApiClient = new ResilientApiClient('local');

// --- FACHADAS DE ESTRATO L7 (USER SERVICES) ---

/** Fachada para la gobernanza financiera y cuotas de silicio. */
export const billingApi = {
  getQuota: () => apiClient.get<BillingQuota>('/user/billing/quota'),
  getHistory: () => apiClient.get<unknown[]>('/user/billing/history'),
};

/** Fachada para el sistema nervioso Herald (Notificaciones). */
export const heraldApi = {
  listNotifications: () => apiClient.get<SystemNotification[]>('/user/herald/notifications'),
  markAsRead: (id: string) => apiClient.post('/user/herald/notifications/read', { notification_identifier: id }),
};

/** Fachada para el motor Nexus de prestigio y red social. */
export const nexusApi = {
  getPrestige: () => apiClient.get<OperatorRank>('/user/nexus/prestige'),
  getLeaderboard: () => apiClient.get<unknown[]>('/user/nexus/leaderboard'),
};

/** Interfaz unificada para consultas de conocimiento y academia. */
export const neuralOracle = {
  query: <T>(query: string, vars?: Record<string, unknown>) => apiClient.graphql<T>(query, vars),
};
