# 📘 CONCEPTO: NEURAL BROADCASTING STRATA

**Clasificación:** SISTEMA DE SEÑALES (ESTRATO L4)
**Misión:** V87.0 - Despacho de Alta Fidelidad

## 1. El Rol del Bus Neural
El `EventBus` actúa como el corazón del "Neural Link". Su misión es desacoplar a los productores (Handlers, Daemons) de los consumidores (WebSockets, Loggers).

## 2. Inyección de Realidad Visual
A partir de la V87.0, el bus se convierte en el transportador oficial de los snapshots del enjambre. Al integrar la imagen base64 directamente en la ráfaga de difusión, garantizamos que todos los operadores conectados al Dashboard Zenith visualicen la misma realidad física del nodo simultáneamente.

## 3. Garantía de Tiempo Constante O(1)
Utilizando el motor de `broadcast` de Tokio, el envío de la señal no se ve penalizado por el número de receptores. Si la imagen es grande, el bus simplemente mueve el puntero de memoria (Arc) hacia los suscriptores, evitando duplicaciones costosas en el heap.
