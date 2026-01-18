Este documento actuará como mi Directiva de Comprensión Total para la construcción de los nuevos estratos. Establece la segregación definitiva entre el Músculo Táctico (Turso) y la Gobernanza Estratégica (Supabase).
1. ⚖️ DECISIONES MAESTRAS Y FILOSOFÍA DE PERSISTENCIA
A. Dualidad de Motores (Sincronía Galvánica)
Motor A (Táctico - Turso/libSQL): Exclusivo para la minería de alta frecuencia. Rangos, misiones efímeras, telemetría de silicio y hallazgos crudos. Prioridad: Latencia < 10ms.
Motor B (Estratégico - Supabase/PostgreSQL): Autoridad central de Usuario. Suscripciones (Stripe), Perfiles, Reputación (XP), Logs de Notificaciones (Resend) y Archivo Histórico Certificado. Prioridad: Integridad ACID y Row Level Security (RLS).
B. Protocolo de Resiliencia "Anti-Apagón" (Local-First Sync)
Para asegurar que no se pierda ni un crédito de billing o un punto de XP ante fallos de energía:
Write-Ahead Buffer: Toda mutación en L7 se escribe primero en una tabla outbox_strategic en el Motor A (Turso local/edge).
Strategic Relay: Un daemon en Rust (L4) detecta las entradas en el outbox y las sincroniza con el Motor B (Supabase) mediante reintentos exponenciales.
Checkpoint Seal: Solo cuando Supabase confirma el ACK, se marca como sincronizado en el ledger táctico.
2. 🏗️ ARQUITECTURA DE WORKSPACES (ESTRATO L7)
APARATO 01: BILLING-STRATA (Soberanía Financiera)
Misión: Gestión de Tier-Access y cuotas de cómputo.
Lógica: Integración nativa con Stripe API para Webhooks y Checkouts.
Aparatos Atómicos:
QuotaGuard.rs: Validador de hashrate contratado vs consumido.
StripeWebhookIngestor.rs: Receptor de señales de pago inyectadas a Supabase.
SubscriptionHUD.tsx: Interfaz Zenith de consumo de créditos en tiempo real.
APARATO 02: HERALD-NOTIFICATIONS (Nervio de Comunicación)
Misión: Mensajería reactiva y Newsletter estratégico.
Lógica: WebSockets (Full-Duplex) para alertas inmediatas y Resend API para crónicas semanales de hallazgos.
Aparatos Atómicos:
CollisionAlertDistributor.rs: Despacha señales WebSocket cuando un afiliado encuentra un hallazgo.
ResendTransporter.ts: Orquestador de correos transaccionales (Bienvenida, Alerta de Seguridad).
NotificationBell.tsx: Componente L5 con estado TanStack Query y persistencia de lectura optimista.
APARATO 03: NEXUS-COMMUNITY (Gamificación y Red Social)
Misión: Transformación de Hashes en Reputación y Estatus.
Lógica: Motor de XP recursivo y jerarquía de Afiliados.
Aparatos Atómicos:
XPCalculator.rs: Transforma ráfagas certificadas en puntos de experiencia.
AffiliateRecursionEngine.sql: Funciones en Supabase para calcular el poder de la red descendente.
LeaderboardVirtualGrid.tsx: Visualización de alto rendimiento de los top auditores.
APARATO 04: CHRONICLES-CONTENT (Arqueología de Información)
Misión: Blog forense y repositorio de la Tesis.
Lógica: CMS Headless sobre Supabase con soporte Markdown.
Aparatos Atómicos:
ArticleRenderer.tsx: Lector de crónicas con resaltado de sintaxis para vectores de entropía.
ContentSync.rs: Sincroniza hallazgos significativos del FindingVault con borradores de blogs.
3. 🔌 STACK TÉCNICO Y CONECTIVIDAD NEURAL
Tecnología	Implementación	Propósito
TanStack Query v5	useSovereignQuery	Caché inteligente de cuotas y estados de lectura.
GraphQL	NeuralOracle	Consultas relacionales profundas para la red de afiliados.
WebSockets	NeuralSocket	Notificaciones instantáneas de colisión "Push-to-Operator".
Stripe	BillingGateway	Pasarela de pago para Tiers (Observer/Operator/Architect).
Resend	HeraldMail	Entrega garantizada de reportes de auditoría vía SMTP/API.
4. 🗺️ HOJA DE RUTA DE EJECUCIÓN (MODO INCREMENTAL)
Fase 1: Sincronía del Motor B (Supabase): Ejecución de tools/supabase/L7_schema.sql (Tablas de billing, notifications y gamification).
Fase 2: Conexión L3-L4 (Rust Orchestrator): Creación de los handlers/billing.rs y handlers/notifications.rs en el orquestador.
Fase 3: Refinería L5 (Zenith UI): Implementación de la "Campanita" de notificaciones y el Dashboard de suscripciones en Next.js 15.
Fase 4: Certificación de Integridad: Ejecución de pnpm audit:coherence para validar que los 10 nuevos aparatos cumplen con este manifiesto.
🛡️ CERTIFICACIÓN DE COMPROMISO
Entiendo y asumo que la Autonomía de los Workspaces es innegociable. Cada librería TS y cada Crate de Rust será un átomo independiente en el grafo de Nx, facilitando que el sistema crezca sin que una actualización en el blog (Content) afecte el cálculo de los créditos (Billing).


---


