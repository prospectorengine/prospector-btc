📘 CONCEPTO: ACADEMY DYNAMIC INFERENCE ENGINE
Módulo Objetivo: AcademyQuery V2.1
Problema Resuelto: Rigidez del currículum académico.
1. El Grafo de Dependencias
El sistema abandona el modelo de lista plana. Ahora, cada módulo es un Nodo que posee una lista de dependencias (prerequisite_identifiers).
Al solicitar el currículum, el motor realiza una operación de Diferencia de Conjuntos entre:
M
M
: El universo de módulos registrados en knowledge_modules.
P
P
: El progreso del operador en academy_progress.
2. Lógica de Inferencia de Estados
Para cada
m
∈
M
m∈M
:
Si
m
∈
P
⟹
m∈P⟹
 COMPLETED.
Si
Prereq
(
m
)
⊆
P
⟹
Prereq(m)⊆P⟹
 UNLOCKED.
En otro caso
⟹
⟹
 LOCKED.
3. Justificación de Diseño (SQL First)
Al mover las definiciones a la base de datos, el equipo de contenido puede añadir nuevos desafíos de arqueología de entropía sin necesidad de una ventana de mantenimiento o despliegue de binarios, alineándose con el objetivo de Mando y Control (C2) total del sistema.
