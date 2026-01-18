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