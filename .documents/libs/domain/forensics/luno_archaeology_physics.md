ARTEFACTO C: EL CONCEPTO (Knowledge)
Ubicación: .documents/libs/domain/forensics/luno_archaeology_physics.md
Física de la Vulnerabilidad: En 2014, ciertas carteras web generaban la clave privada mediante un proceso pseudo-aleatorio dependiente del reloj del sistema. La entropía real se redujo de 256 bits a un rango de aproximadamente 32-40 bits (milisegundos transcurridos en una ventana temporal específica).
Mecánica de Reconstrucción: El iterador recibe un start_timestamp_ms y un end_timestamp_ms. Por cada tick, el sistema:
Transforma el milisegundo en un buffer de bytes.
Aplica SHA256 para materializar el escalar privado.
Valida si el escalar reside en el grupo de la curva.
Justificación de Diseño: Se utiliza un iterador de Cero Alocación que genera tuplas (String, SafePrivateKey). La cadena de texto (metadata) permite al operador rastrear el milisegundo exacto de la colisión en el Dashboard Zenith, facilitando la datación forense del hallazgo.
