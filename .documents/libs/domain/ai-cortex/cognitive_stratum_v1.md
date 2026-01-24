🧩 ARTEFACTO C: EL CONCEPTO (Knowledge)
Ubicación: .documents/libs/domain/ai-cortex/cognitive_stratum_v1.md
Física del AI Cortex: El aparato opera como un Subsistema Cognitivo Desacoplado. Su arquitectura se basa en el patrón de "Observador Inteligente": se suscribe al EventBus (L4), asimila los SystemLog y el SystemMetrics, y genera un DecisionVector.
Soberanía de Cómputo: Al ser un workspace independiente, podemos escalar sus dependencias (como tch para PyTorch o clientes de LLM como Gemini/OpenAI) sin contaminar el binario ligero del miner-worker.
Bucle de Retroalimentación: El Cortex no solo "mira", sino que propone. Sus salidas son OptimizationDirectives que el OperationalNexus puede validar antes de aplicarlas a la flota, garantizando una autonomía controlada por el Arquitecto.
