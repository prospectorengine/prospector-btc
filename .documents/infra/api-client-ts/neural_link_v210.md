Física de la Reconciliación:
En la V210.0, cada snapshot visual (vr) forzaba un escaneo lineal del array de flota. En un enjambre de 300 nodos, esto representaba 300 operaciones de búsqueda por cada ciclo de refresco. Al transicionar a fleet_map_ref (Map), la actualización es instantánea (
O
(
1
)
O(1)
). El uso de un temporizador de compromiso (COMMIT_INTERVAL_MS) protege el hilo principal de JavaScript, permitiendo que la UI respire mientras la red satura el buffer de memoria.
