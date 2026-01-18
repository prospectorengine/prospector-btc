CONCEPTO: DYNAMIC ACADEMY SCHEMA
Módulo Objetivo: DatabaseSchema V151.0
Problema Resuelto: Desacoplamiento del currículum técnico del código fuente.
1. El Sustrato de Conocimiento
La tabla knowledge_modules actúa como el Cerebro del Oráculo. Almacena la topología del grafo de aprendizaje.
Prerequisites: Almacenado como un string plano (CSV), lo que permite resoluciones rápidas mediante split(',') en el resolver de Rust, minimizando la complejidad de las uniones (joins) en SQLite.
I18N Mapping: Las columnas i18n_* aseguran que el Orquestador permanezca agnóstico al lenguaje humano, delegando la visualización al Dashboard Zenith (L5).
2. Garantía de No-Regresión
El uso de solidify_base_strata asegura que las nuevas tablas no interfieran con las ya existentes. El mecanismo de execute_evolutionary_repair protege el historial de misiones del enjambre al ignorar errores de "columna duplicada" si el script se ejecuta múltiples veces.
3. Topología Relacional
knowledge_modules (Definición)
←
←
 academy_progress (Estado del Operador).
Esta relación permite al sistema calcular en tiempo real qué módulos están LOCKED o UNLOCKED basándose en la intersección de ambos sustratos.
