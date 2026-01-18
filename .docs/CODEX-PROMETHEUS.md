# 🔥 CODEX PROMETHEUS: LA ARQUITECTURA DE LA IA SOBERANA
**Clasificación:** FUTURO (ROADMAP V20.0+)
**Objetivo:** Transición de un Sistema Automatizado a un Organismo Autónomo.
**Motor Objetivo:** Gemini 23.0+ / Modelos de Razonamiento Avanzado.

---

## 1. VISIÓN ESTRATÉGICA: LA SINGULARIDAD OPERATIVA

Actualmente, el sistema `Prospector` es reactivo: los humanos leen logs y ajustan parámetros.
El **Proyecto Prometheus** define la arquitectura para cerrar el bucle:
**`Ejecución -> Percepción -> Razonamiento (IA) -> Optimización -> Ejecución`**

El objetivo final es que el algoritmo no solo busque claves, sino que se reescriba y reconfigure a sí mismo para maximizar la eficiencia termodinámica y criptográfica.

---

## 2. ARQUITECTURA DE APARATOS NEURALES

Para soportar esta visión, el sistema debe evolucionar añadiendo cuatro nuevos órganos vitales.

### A. EL OJO: `Heimdall-Cortex` (Percepción Semántica)
Evolución del logger actual (`heimdall-ts` / `tracing-rs`).
*   **Cambio de Paradigma:** De "Texto Plano" a "Vectores de Estado".
*   **Estructura del Dato:** Cada log debe capturar el contexto completo del universo en ese milisegundo (Temperatura CPU, Presión de Memoria, Latencia de Red, Hashrate Instantáneo).
*   **Destino:** No un archivo `.log`, sino un **Vector Store** (Supabase pgvector / Pinecone).

### B. LA MEMORIA: `The Synapse` (Base de Datos Vectorial)
Repositorio histórico de comportamiento y causalidad.
*   **Función:** Almacenar "Recuerdos Operativos".
*   **Ejemplo de Consulta IA:** *"Busca patrones donde el Hashrate cayó >15% y correlaciónalos con la versión del Kernel de Linux del Worker"*.
*   **Tecnología:** Supabase `neural_logs` table (JSONB + Vector Embeddings).

### C. EL CEREBRO: `The Strategist` (Agente Autónomo)
Servicio externo (Cloud Function / Container) donde reside Gemini.
*   **Ciclo de Vida:**
    1.  **Ingesta:** Lee los últimos 5 minutos de `Heimdall-Cortex`.
    2.  **Inferencia:** Detecta ineficiencias (ej: "Cuello de botella en la serialización JSON").
    3.  **Decisión:** Genera una `OptimizationDirective`.

### D. LA MANO: `The Effector` (API de Control Dinámico)
Interfaz segura en el Orquestador (Rust) que permite modificar el comportamiento en tiempo de ejecución.
*   **Mecanismo:** `Hot-Reloadable Config`.
*   **Perillas Exponibles:**
    *   `batch_size`: Tamaño de ráfaga de claves.
    *   `thread_affinity`: Asignación de núcleos.
    *   `strategy_aggression`: Intensidad de búsqueda vs enfriamiento.

---

## 3. ESTRUCTURA DEL "NEURO-LOG" (CONTRATO FUTURO)

Todo componente nuevo debe ser capaz de emitir este formato:

```json
{
  "timestamp": 1736284000,
  "signal_id": "uuid-v4",
  "stratum": "L1_CORE_MATH",
  "event_type": "PERFORMANCE_SAMPLE",
  "context_snapshot": {
    "cpu_temp_c": 78.5,
    "ram_usage_mb": 4096,
    "active_threads": 8,
    "network_latency_ms": 45
  },
  "execution_trace": {
    "module": "secp256k1.rs",
    "function": "double_deterministic",
    "avg_cycles_per_op": 120
  },
  "outcome_metric": 0.85 // (Normalizado 0.0 - 1.0)
}

4. METODOLOGÍA DE "OPTIMIZACIÓN RECURSIVA"
El sistema operará bajo el principio de A/B Testing Continuo Autónomo.
Hipótesis (IA): "Reducir el batch_size a 512 podría mejorar la latencia en nodos con poca RAM".
Experimento: El Strategist ordena al Effector aplicar batch_size=512 al 10% del enjambre.
Observación: Heimdall-Cortex registra el rendimiento comparativo.
Conclusión: Si el rendimiento sube, la IA aplica el cambio globalmente. Si baja, revierte y "aprende" (guarda el vector de fallo en The Synapse).
5. REGLAS PARA EL DESARROLLO ACTUAL (COMPATIBILIDAD)
Para no bloquear este futuro, todo código escrito hoy debe seguir estas directivas:
Observabilidad Estructurada: Nunca usar console.log("Error"). Usar siempre logger.error("Msg", { metadata }). La metadata es el alimento de la IA.
Configuración Externa: Evitar "Hardcoding" de constantes mágicas (ej: const BATCH = 1000). Moverlas a configuración inyectable o variables de entorno para que The Effector pueda tocarlas mañana.
Manejo de Errores Rico: Los errores deben tener code, context y stack, no solo mensajes de texto.


---

