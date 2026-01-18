 CONCEPTO: PANOPTICON VIRTUALIZATION ENGINE
Módulo Objetivo: SystemLogConsole V2.1
Problema Resuelto: "DOM Bloat" y degradación de memoria en sesiones de monitoreo prolongadas.
1. Física de la Virtualización
En la versión 2.0, el uso de content-visibility: auto optimizaba el pintado, pero el navegador aún mantenía 5,000 objetos de tipo div en el árbol de accesibilidad y memoria.
En la V2.1, implementamos un Windowing Manual:
Se calcula la altura total del scroll de forma teórica:
B
u
f
f
e
r
×
R
o
w
H
e
i
g
h
t
Buffer×RowHeight
.
Se genera un "recorte" (Slice) de los datos basado en el scrollTop.
Solo los elementos contenidos en ese recorte se instancian como nodos del DOM.
2. Topología del Buffer Circular
El Orquestador (L3) emite ráfagas de hasta 100 logs/seg. El componente recibe estas ráfagas y las acumula en un buffer de máximo 5,000 registros.
Inserción:
O
(
1
)
O(1)
 mediante slice(-MAX).
Renderizado:
O
(
w
i
n
d
o
w
)
O(window)
 donde window es constante (~100), eliminando la correlación entre el tamaño del buffer y la carga de la CPU del Dashboard.
3. Justificación de Diseño
Se ha evitado el uso de librerías como react-window para mantener la Soberanía del Código. La lógica de virtualización es pura, lineal y predecible, facilitando la auditoría de la Tesis sin dependencias de terceros que podrían introducir rastro de tracking o ineficiencias de bundle.
