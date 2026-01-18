📜 PROTOCOLO DE ALINEACIÓN ESTRATÉGICA: EL PUENTE DE RESILIENCIA (V2026.2)
ARQUITECTO: AI SYSTEMS COMMANDER
Comandante, la nueva alineación arquitectónica de Motores Gemelos con Patrón Outbox no es un cambio cosmético; es la implementación de un Mecanismo de Supervivencia de Datos de grado militar. A continuación, presento la justificación técnica, el punto de bitácora y la actualización del manifiesto para sellar esta decisión.
⚖️ 1. JUSTIFICACIÓN TÉCNICA: ¿POR QUÉ ESTA ALINEACIÓN?
En la arquitectura previa, el Orquestador intentaba hablar con Supabase (Motor B) mientras gestionaba a los Workers. Si Supabase presentaba latencia (común en ráfagas masivas), el Orquestador se bloqueaba, degradando el hashrate global.
Beneficios de la nueva alineación:
Desacoplamiento Térmico/Lógico: El "calor" de la minería (Turso) se separa de la "frialdad" de la administración (Supabase). Turso maneja el caos de los 120MH/s; Supabase custodia el valor inmutable del usuario.
Inmunidad ante Latencia de Nube: Al escribir en el outbox_strategic de Turso (Local/Edge), la latencia es < 5ms. El usuario recibe confirmación instantánea, mientras que el envío real a Supabase ocurre en segundo plano.
Garantía "At-Least-Once": Si hay un corte de energía o de red, los créditos de Billing y el XP no se pierden; están sellados en el silicio de Turso. Al reiniciar, el StrategicRelay retoma el vaciado desde el último ID procesado.
Saturación del Pipeline: Permite que el Orquestador procese miles de handshakes por segundo sin esperar a que PostgreSQL (Supabase) confirme transacciones pesadas.

---


