==================== INICIO DEL ARCHIVO [.docs/manifiesto-de-pruebas.md] ====================
🛡️ MANIFIESTO DE CERTIFICACIÓN DE INTEGRIDAD (V2.0)
Clasificación: PROTOCOLO SOBERANO // TRINITY COMPLIANT
Objetivo: Garantizar Cero Regresiones y Verdad Matemática en el enjambre Hydra-Zero.
1. FILOSOFÍA DE LA PRUEBA (EL TRIÁNGULO DE HIERRO)
En Prospector, no "testeamos para ver si funciona", certificamos que es matemáticamente imposible que falle bajo las leyes de la criptografía.
A. Estrato de Unidad (L1 - La Célula)
Motores Rust: Pruebas unitarias deterministas con cobertura del 100% en math-engine y generators.
Frontera TS: Validación de esquemas Zod y pureza de los adaptadores de API.
Herramientas: cargo test, jest.
B. Estrato de Integridad (L2/L3 - El Sistema Nervioso)
Property-Based Testing (Fuzzing): Uso de proptest en Rust para bombardear el motor secp256k1 con billones de escalares aleatorios buscando fallos de borde.
Paridad Cross-Platform: El VaultCryptoEngine debe producir el mismo CipherText en Rust y en TypeScript. Si hay una diferencia de un bit, el sistema se bloquea.
Herramientas: proptest, test-containers.
C. Estrato de Resiliencia (L4/L6 - La Guerra)
Chaos Engineering: Simulación de desconexión de base de datos Turso a mitad de una ráfaga de Montgomery. El sistema debe realizar un Atomic Rollback sin perder el checkpoint.
E2E Visual: Playwright certificando que el Dashboard visualiza correctamente las colisiones inyectadas por el Prover.
Herramientas: playwright, docker-compose stress.
2. INVENTARIO DE CERTIFICACIÓN POR WORKSPACE
L1_CORE_MATH (Prioridad: CRÍTICA)

Field Integrity Torture: Validar adición y multiplicación modular contra num-bigint (Oráculo).

Jacobian Parity: Certificar que P + G es idéntico a G * (k+1) usando duplicación escalar.

Montgomery Batch Accuracy: Validar que batch_invert de 1024 elementos es bit-perfect vs inversos individuales.
L3_INFRA_DB (Prioridad: ALTA)

ACID Lease Protection: Validar que dos hilos pidiendo la misma identidad reciban error de bloqueo (No-Collision Lease).

Chronos Drift Audit: Validar que el reporte de misiones entre Turso y Supabase tiene un drift < 0.01%.
L5_WEB_DASHBOARD (Prioridad: MEDIA)

Neural Link Reconnect: Validar que el stream SSE se recupera automáticamente tras 10 segundos de corte de red.

ZK-Vault Zero-Leak: Certificar que el campo credentials_json nunca viaja al servidor en texto plano.
3. PROTOCOLO DE EJECUCIÓN (WIN-10)
Todo cambio debe ser precedido por la "Purga de Regresiones":
code
Powershell
# Nivelación L1/L2
cargo test --workspace --release

# Nivelación L4/L5
pnpm audit:logic
==================== FIN DEL ARCHIVO [.docs/manifiesto-de-pruebas.md] ====================


📜 MANDATO SUPREMO DE VALIDACIÓN (PROMPT MEJORADO)
"Actúa como Arquitecto de Integridad de Sistemas. Tu misión es certificar la soberanía de cada aparato mediante el Protocolo Trinidad Nivelado.
1. EXHAUSTIVIDAD TOTAL: La prueba debe cubrir:
Lógica Nominal: Casos de éxito bit-perfectos.
Lógica de Frontera: Desbordamientos, singularidades y errores de acarreo.
Rendimiento: Benchmark de throughput (ops/seg) bajo carga real.
2. VERBOSIDAD FORENSE: La ejecución debe imprimir en consola un diario de operaciones en Español, detallando cada fase del escaneo.
3. PERSISTENCIA DE EVIDENCIA: El test debe generar o sobrescribir un informe en reports/qa/[nombre_aparato]_report.json con metadatos técnicos completos.
4. SIN REGRESIONES: El código debe ser un superconjunto funcional, respetando la ruta espejo tests/mirror/ y garantizando que las dependencias estén perfectamente mapeadas."
