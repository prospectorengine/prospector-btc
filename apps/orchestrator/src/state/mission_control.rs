// INICIO DEL ARCHIVO [apps/orchestrator/src/state/mission_control.rs]
//! =================================================================
//! APARATO: MISSION CONTROL MANAGER (V151.0 - ROLLBACK CAPABLE)
//! CLASIFICACIÓN: APPLICATION STATE (ESTRATO L1-APP)
//! RESPONSABILIDAD: GESTIÓN DE INVENTARIO CON CAPACIDAD DE REVERSIÓN
//! =================================================================

use std::collections::VecDeque;
use std::sync::Mutex;
use prospector_domain_models::work::WorkOrder;

pub struct MissionControlManager {
    /// Cola FIFO de misiones listas para ser consumidas por el enjambre.
    /// El acceso está protegido por un Mutex para garantizar la atomicidad del 'Pull'.
    active_dispatch_queue: Mutex<VecDeque<WorkOrder>>,
}

impl MissionControlManager {
    pub fn new() -> Self {
        Self {
            active_dispatch_queue: Mutex::new(VecDeque::with_capacity(1000)),
        }
    }

    /**
     * Extrae una misión de la cola de despacho.
     * Operación O(1). No requiere transacciones de base de datos.
     */
    pub fn pull_assignment(&self) -> Option<WorkOrder> {
        let mut queue_guard = self.active_dispatch_queue.lock().expect("Mission Queue Poisoned");
        queue_guard.pop_front()
    }

    /**
     * Inyecta un lote de misiones pre-asignadas en la cola.
     */
    pub fn hydrate_queue(&self, batch: Vec<WorkOrder>) {
        let mut queue_guard = self.active_dispatch_queue.lock().expect("Mission Queue Poisoned");
        queue_guard.extend(batch);
    }

    /**
     * ✅ NUEVO: Devuelve una misión fallida al frente de la cola (Alta Prioridad).
     * Crítico para el manejo de fallos de persistencia en el Handshake.
     */
    pub fn rollback_mission(&self, mission: WorkOrder) {
        let mut queue_guard = self.active_dispatch_queue.lock().expect("Mission Queue Poisoned");
        queue_guard.push_front(mission);
    }

    /**
     * Retorna la cantidad de misiones remanentes en el buffer de memoria.
     */
    pub fn get_available_buffer_size(&self) -> usize {
        let queue_guard = self.active_dispatch_queue.lock().expect("Mission Queue Poisoned");
        queue_guard.len()
    }
}

impl Default for MissionControlManager {
    fn default() -> Self {
        Self::new()
    }
}
// FIN DEL ARCHIVO [apps/orchestrator/src/state/mission_control.rs]
