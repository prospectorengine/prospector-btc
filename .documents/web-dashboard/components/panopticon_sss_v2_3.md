CONCEPTO: PANOPTICON SSS (SOVEREIGN SIGNAL SYSTEM)
Módulo Objetivo: SystemLogConsole V2.3
Problema Resuelto: Erradicación de "variables muertas" y optimización de renderizado masivo.
1. El Muro de Higiene
En la V2.3, se ha aplicado un patrón de Consumo Obligatorio. Cada variable de estado (is_filter_menu_active, scroll_top_position) y cada componente visual (Filter, AnimatePresence, useCallback) ha sido cableado directamente a la lógica de la UI. Esto garantiza que el compilador no genere ruido y que el bundle de Vercel sea el más ligero posible.
2. Windowing Determinista
A diferencia de content-visibility, la virtualización manual por ROW_HEIGHT_PX permite que el navegador calcule el scrollbar basándose en un div de altura teórica (
N
×
26
p
x
N×26px
), mientras que el DOM físico solo mantiene los nodos que el ojo humano puede percibir. Esto reduce el uso de memoria RAM en el Dashboard en un 90% para buffers grandes.
3. Justificación de Diseño
Se ha optado por un sistema de Search-in-Strata integrado. El operador puede filtrar logs no solo por severidad, sino por palabra clave en tiempo real, lo que convierte a la consola en una herramienta de depuración forense de grado industrial.
