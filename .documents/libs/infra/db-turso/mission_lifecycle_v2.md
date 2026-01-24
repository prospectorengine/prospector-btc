Ubicación: .documents/libs/infra/db_turso/mission_lifecycle_v2.md
Física del Aparato: Este test certifica la confiabilidad del Ledger Táctico (Motor A). En un sistema distribuido de alta frecuencia, el riesgo de que dos nodos minen el mismo rango es alto. Validamos que la base de datos Turso actúe como un semáforo atómico, garantizando la exclusividad de cada misión.
Mecánica ACID: Se pone a prueba la cláusula WHERE status = 'queued' en la operación UPDATE. Esto garantiza que si una misión ya cambió a active, cualquier otro intento de actualización será ignorado (rows_affected == 0), disparando una alerta de colisión de propiedad.
Trazabilidad de Latencia: El test mide el RTT de las operaciones INSERT y UPDATE. Si el tiempo de respuesta de Turso excede los 500ms en el entorno de pruebas, el Dashboard reportará un estado DEGRADED, alertando sobre posibles cuellos de botella en la red del orquestador.


---

