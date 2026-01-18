Física del Desacoplamiento:
Cargo impone que un archivo de código solo puede pertenecer a un "árbol de módulos" a la vez. Al definir lib.rs, creamos una unidad de compilación reutilizable. El binario main.rs y los tests de integración se convierten en consumidores externos, eliminando la redundancia y los conflictos de símbolos que disparaban el warning en su VAIO.
