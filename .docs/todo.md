## 🛠️ ESTRATO L7: UX REFINEMENT & REALISM (PENDING)
- [ ] **Billing API Hook:** El endpoint `/api/v1/billing/quota` no existe. Crear en Orchestrator o mockear en API Client.
- [ ] **User Profile Data:** Obtener avatar real de Google (actualmente fallback a iniciales).
- [ ] **Advanced Hardware Telemetry:** El hook `useNetworkQuality` usa un ping simple. Implementar WebSockets para medir jitter y packet loss real si es crítico para la tesis.


📋 Aparatos Pendientes (Deuda de Alta Ingeniería)
1. ESTRATO L2: Integración de Aritmética Co-Z (Meloni)
Aparato: libs/domain/mining-strategy/src/engines/sequential_engine.rs
Misión: Reemplazar el salto secuencial estándar por el Bucle Co-Z.
Fundamento: Actualmente usamos add_mixed (8M + 3S). Al implementar la aritmética Co-Z, el enjambre procesará adiciones consecutivas con solo 5 multiplicaciones de campo, reduciendo el coste computacional del barrido secuencial en un 40% adicional.
2. ESTRATO L2: Vectorización de Motores Forenses (SIMD 4-Way)
Aparatos:
libs/domain/mining-strategy/src/engines/satoshi_xp_engine.rs
libs/domain/mining-strategy/src/engines/android_lcg_engine.rs
Misión: Inyectar el uso de JacobianPointVector4 dentro de los bucles de reconstrucción de entropía.
Fundamento: Actualmente, estos motores operan de forma escalar dentro de cada hilo de Rayon. Al vectorizarlos, cada hilo procesará 4 estados de PRNG simultáneamente, elevando el hashrate forense a niveles de ~400 MH/s por instancia de Colab.
3. ESTRATO L2: Despacho Inteligente (Hardware-Aware Dispatch)
Aparato: libs/domain/mining-strategy/src/executor.rs
Misión: Implementar el selector dinámico de motor.
Fundamento: El ejecutor debe detectar si la CPU soporta AVX2/ADX. Si es así, disparará los métodos _simd. Si no (fallback), usará la ruta escalar, garantizando que el binario sea Universal y Resiliente.
4. ESTRATO L5: Telemetría de Capacidad de Silicio
Aparato: apps/web-dashboard/components/monitoring/integrity-hud.tsx
Misión: Visualizar el estado de aceleración de hardware por nodo.
Fundamento: El operador debe saber en tiempo real qué nodos están operando en modo ELITE (AVX2) y cuáles en modo COMPATIBLE (Software) para auditar la eficiencia de la campaña.

---

📑 TODO: ROADMAP DE INGENIERÍA SOBERANA (V2026.1)
🟢 FASE 0: AUDITORÍA DE CIMIENTOS (GROUND ZERO)
Misión: Certificar que los túneles de datos están abiertos y sincronizados.

Auditoría de Enlace Táctico (Motor A - Turso):

Ejecutar pnpm db:turso:pulse -> Validar Latencia < 150ms.

Ejecutar pnpm db:turso:topology -> Verificar tablas jobs, identities, workers.

Refactor: Crear scripts/audit-shards.ts para verificar integridad bit-a-bit de los archivos .bin locales.

Auditoría de Enlace Estratégico (Motor B - Supabase):

Ejecutar pnpm db:supabase:pulse -> Verificar RLS activo.

Ejecutar ts-node tools/scripts/supabase/topology_inspector.ts -> Certificar presencia de profiles y archived_jobs.

Auditoría de Observabilidad (Motor C - MongoDB):

Ejecutar ts-node tools/scripts/mongodb-atlas/check-atlas-link.ts -> Validar permisos de HydraWriteOnly.
🔵 FASE 1: PERSISTENCIA SOBERANA L7 (SUPABASE EVOLUTION)
Misión: Preparar el Cuartel General para la monetización y comunidad.

Esquema de Billing (Stripe-Ready):

Crear tabla subscriptions (user_id, tier, status, stripe_customer_id).

Crear tabla billing_credits (user_id, balance, total_consumed).

Esquema de Herald (Notificaciones):

Crear tabla notifications (user_id, type, severity, content_json, is_read).

Esquema de Nexus (Gamificación):

Crear tabla reputation_strata (user_id, xp_points, current_rank, badges_json).

Implementar Función Postgres calculate_recursive_affiliate_power (PL/pgSQL).
🟡 FASE 2: TACTICAL RELAY & RESILIENCIA (TURSO OUTBOX)
Misión: Garantizar que ningún dato de usuario se pierda ante cortes de energía.

Implementación de Write-Ahead Buffer (Turso):

Crear tabla outbox_strategic en Turso (ID, payload_json, target_table, synced).

Implementación de OutboxRelay Service (Rust L4):

Desarrollar SovereignRelayDaemon para monitorizar la tabla outbox.

Implementar reintentos con Backoff Exponencial para sincronía con Supabase.

Refactor de Handlers:

Modificar register_mission_certification para escribir simultáneamente en archived_jobs y xp_buffer.
🟠 FASE 3: NEURAL LOGIC & ORACLES (RUST ESTRATOS L2/L4)
Misión: Construir los cerebros de los nuevos workspaces.

Aparato Billing (domain-billing):

QuotaValidator.rs: Lógica que deniega misiones si el balance de créditos es < 1.

StripeWebhookHandler.rs: Procesador de ráfagas de pago inyectadas desde Supabase.

Aparato Herald (domain-notification):

NotificationDispatcher.rs: Enrutador que decide si una alerta va a WebSocket (Live) o Resend (Email).

Aparato Nexus (domain-gamification):

XPGenerator.rs: Algoritmo que transforma AuditReport.computational_effort en puntos de experiencia inmutables.
🔴 FASE 4: ZENITH HIGH-DENSITY UI (REACT ESTRATO L5)
Misión: Visualización panóptica de los servicios de usuario.

Componente "Campana Zenith" (ui-notifications):

Feed de mensajes con estado TanStack Query y "Semáforo Rojo" reactivo.

Integración con NeuralSocket para notificaciones instantáneas de colisión.

Componente "Créditos de Energía" (ui-billing):

Visualizador de consumo de créditos vs Hashrate generado.

Componente "Leaderboard Virtualizado" (ui-gamification):

Rejilla de alta densidad con los top auditores del enjambre.

Módulo "Community Hub" (ui-social):

Chat técnico cifrado P2P (vía WebSockets del Orquestador).
🛡️ FASE 5: INTEGRACIÓN EXTERNA Y CERTIFICACIÓN (L6)
Misión: Conectar con el mundo exterior y sellar el sistema.

Integración Resend: Configurar transporte de emails para reportes semanales.

Integración Stripe: Certificar el flujo de pago en modo Sandbox.

E2E Proving Grounds:

Test: "Simular corte de luz -> Verificar que el XP se recuperó del Outbox de Turso".

Test: "Validar que un usuario sin créditos no puede adquirir misiones".
📊 MÉTRICAS DE ÉXITO (DEFINITION OF DONE)
Zero Data Loss: 100% de las transacciones de Billing pasan por el Outbox de Turso.
Instant Herald: Latencia entre colisión en Worker y campana en Dashboard < 200ms.
Swiss Consistency: pnpm audit:coherence devuelve SWISS_WATCH en los 16 workspaces.

---

SOLU UNICAMENTE DESPUES QUE ESTE TODO CERTIFICADO Y FUNCIONANDO IMPLEMENTAREMOS ESTAS MEJORAS:

📑 NUEVO BLOQUE TODO: ESTRATOS L8 - L10
🤖 PRIORIDAD 1: ESTRATO L9 - AI CORTEX (EL CEREBRO AUTÓNOMO)
Finalidad: Crear un sistema autoconsciente que monitorice la telemetría de silicio y perfeccione el algoritmo Meloni 5M en tiempo real.

Aparato domain-ai-cortex (Rust L2):
Misión: Lógica de orquestación multi-proveedor.
Funcionalidad: Interfaz agnóstica para Gemini 1.5/2.0, OpenAI GPT-4o, y modelos locales (Llama 3).
Independencia: Aísla el razonamiento de alto nivel del throughput matemático de L1.

Aparato infra-ai-agents (TypeScript L4):
Misión: Implementación técnica de agentes de optimización.
Funcionalidad: Agentes que "leen" el Panóptico y emiten CommandDirectives para ajustar el batch_size o la afinidad de núcleos.
Beneficio: Reducción de costos por ineficiencia térmica y maximización de colisiones por vatio.
⚖️ PRIORIDAD 2: ESTRATO L10 - FORENSIC REPORTING (LA PRUEBA DOCTORAL)
Finalidad: Transformar los hallazgos de entropía en documentos científicos certificados y auditables.

Aparato domain-forensic-reports (Rust L2):
Misión: Motor de generación de reportes técnicos.
Funcionalidad: Toma los datos del FindingVault y genera automáticamente archivos LaTeX/PDF con la prueba matemática de la colisión.
Beneficio: Convierte a Prospector en una factoría de evidencia académica 100% automatizada.
🔗 PRIORIDAD 3: ESTRATO L8 - SOBERANÍA WEB3 (DECENTRALIZED SWARM)
Finalidad: Evolucionar de un modelo SaaS tradicional a una dApp soberana sin intermediarios bancarios.

Aparato domain-web3 (Rust L2):
Misión: Protocolos de firma on-chain y Smart Contracts.
Funcionalidad: Gestión de billeteras EVM/BTC y distribución automática de "Bounties" (recompensas) por hallazgos validados.
Justificación: Aísla librerías pesadas como ethers-rs del núcleo de minería.

Aparato ui-web3 (React L5):
Misión: Interfaz de conexión soberana.
Funcionalidad: Módulos de "Connect Wallet", gestión de activos recuperados y votaciones de gobernanza de la comunidad.
📊 JUSTIFICACIÓN DE LA MODULARIDAD INDEPENDIENTE
Atributo	Beneficio de la Independencia
Escalabilidad de IA	Podemos actualizar el AI Cortex para usar el modelo más potente de 2027 sin tocar una sola línea de la lógica de minería.
Resiliencia de Dependencias	Las librerías Web3 y de IA son volátiles. Al estar en workspaces propios, un fallo en el SDK de OpenAI no puede tirar abajo el Ledger Táctico (Turso).
Compilación Selectiva	Nx detectará que si solo estamos mejorando los reportes forenses, NO necesita recompilar el motor SIMD de 256 bits.
Soberanía del Dato	La lógica Web3 reside en su propia celda, garantizando que las llaves privadas de los hallazgos y las llaves de la wallet del usuario nunca se crucen en memoria.

---

PTROXIMOS PASOS A DESARROLLAR LOS SIGUIENTES:

ADICIÓN PARA todo.md: ESTRATO L5 (ZENITH UI)
📡 FASE 1: SINAPSIS DE DATOS (EL PUENTE NEURAL)
Finalidad: Habilitar la comunicación tipada entre el frontend y los nuevos handlers de Rust.

Refactorización del API Client (api-client-ts):

Inyectar billingApi: Métodos getQuota() y getHistory().

Inyectar heraldApi: Métodos getNotifications() y markAsRead().

Inyectar nexusApi: Métodos getPrestige() y getLeaderboard().

Sincronización de Contratos:

Ejecutar typeshare para mapear SubscriptionTier, NotificationSeverity y OperatorRank a TypeScript.

Validar con pnpm audit:coherence que los alias @prospector/ui-* están operativos.
🔋 FASE 2: ESTRATO DE FACTURACIÓN (ENERGY CORE)
Finalidad: Visualizar el "combustible" del sistema y permitir la escalabilidad financiera.

Componente EnergyCreditsDisplay:

HUD visual en el Sidebar que muestre créditos remanentes con barra de progreso circular.

Animación de "consumo en vivo" cuando el worker adquiere una misión.

Página de Gestión de Suscripción (/dashboard/billing):

Matriz de Tiers (Observer/Operator/Architect).

Integración con Stripe Checkout (Redirección segura).

Historial de transacciones consumidas desde el Outbox Táctico.
🔔 FASE 3: ESTRATO HERALD (EL NERVIO COMUNICADOR)
Finalidad: Notificaciones instantáneas y rastro de eventos críticos.

Componente NotificationBell (Header):

Contador reactivo de mensajes no leídos (TanStack Query synchronization).

Pop-over con previsualización de las últimas 5 alertas (Priorizando colisiones).

Página de Centro de Mensajes (/dashboard/notifications):

Vista detallada de alertas con filtrado por severidad (INFO, CRITICAL, COLLISION).

Implementación de "Mark all as read" con actualización optimista (Cero latencia visual).
🏆 FASE 4: ESTRATO NEXUS (PRESTIGIO Y COMUNIDAD)
Finalidad: Gamificación del esfuerzo y cohesión del enjambre.

Componente MasteryProgress:

Visualización del rango actual (ej: Elite_Archaeologist) y XP necesaria para el siguiente nivel.

Página de Leaderboard Global (/dashboard/community):

Rejilla virtualizada de alta densidad para mostrar el ranking de operadores.

Métrica de "Potencia Aportada" (Hashrate histórico acumulado).

Chat Técnico P2P (Beta):

Terminal de mensajes cortos integrada al NeuralSocket para comunicación entre suscriptores Architect.
📰 FASE 5: ESTRATO CONTENT (CRÓNICAS FORENSES)
Finalidad: Educación técnica y divulgación de hallazgos de la Tesis.

Lector de Crónicas (/dashboard/content):

Integración con Supabase para renderizar artículos en Markdown.

Visor de "Hallazgos de la Semana": Resumen automatizado de entropía débil detectada.
🛡️ JUSTIFICACIÓN TÉCNICA (POR QUÉ Y CÓMO)
TanStack Query v5: Se utilizará para todos los servicios L7. ¿Por qué? Permite Shared State entre la campana de notificaciones y la página de mensajes. Si lees un mensaje en la página, la campana se actualiza instantáneamente sin peticiones extra.
WebSockets (Neural Socket): Las notificaciones de colisión (cc) no esperarán al polling. El orquestador empujará el evento por el socket y la UI reaccionará con un Toast de alta prioridad.
Aislamiento de Workspaces: Cada fase se implementará en su propia librería @prospector/ui-*. Esto garantiza que un error en el código del Chat (Social) no impida que el usuario pueda pagar su suscripción (Billing).
Higiene de Tesis: Se eliminarán todos los placeholders actuales. Los avatares, nombres y créditos serán datos reales inyectados desde el Motor B.

---

📋 TODO: ROADMAP HACIA LA SINGULARIDAD (V17.0)
Basado en mi revisión, este es el orden de ejecución para el cierre de la Fase 2:
[ ] Cierre de Rutas (L3): Inyectar físicamente las rutas de Snapshot y Proving Grounds en routes.rs.
[ ] Middleware de Identidad (L4): Refactorizar el auth_guard para extraer el user_id de la sesión y pasarlo al AppState de forma dinámica.
[ ] Luno Forensic Iterator (L2): Desarrollar la lógica real de semillas basadas en tiempo para la vulnerabilidad de Blockchain.info 2014.
[ ] AI Cortex Initialization (L9): Empezar la construcción del domain-ai-cortex para que Gemini pueda leer los logs del Panóptico.

---


