// [apps/miner-worker/src/cpu_manager.rs]
/**
 * =================================================================
 * APARATO: HEURISTIC HARDWARE MONITOR (V131.0 - SOBERANO)
 * CLASIFICACIÓN: WORKER INFRASTRUCTURE (ESTRATO L1-WORKER)
 * RESPONSABILIDAD: TELEMETRÍA DE SILICIO CON ADAPTACIÓN VIRTUAL
 *
 * # Logic:
 * En infraestructuras efímeras (Colab/Kaggle), el acceso a sensores
 * térmicos suele estar capado por el hipervisor. Este aparato:
 * 1. Intenta acceso directo a descriptores hwmon de Linux.
 * 2. Si fallan, calcula una "Temperatura Sintética" basada en la
 *    curva de carga de la CPU (Heurística de Presión).
 * 3. Garantiza que el Dashboard Zenith siempre visualice actividad.
 *
 * # Performance:
 * Complejidad O(1). Las lecturas de /proc son pseudo-archivos en memoria,
 * con un impacto despreciable en el ciclo de minería Jacobiana.
 * =================================================================
 */

use std::fs;

/// Estructura de telemetría de hardware de alta resolución.
#[derive(Debug, Clone, Copy)]
pub struct NodeHardwareMetrics {
    /// Frecuencia actual del procesador en MHz.
    pub cpu_frequency_megahertz: u32,
    /// Promedio de carga del sistema (1 minuto).
    pub system_load_average: f32,
    /// Temperatura detectada o estimada del núcleo.
    pub core_temperature_celsius: f32,
    /// Uso actual de memoria RAM en bytes.
    pub memory_utilization_bytes: u64,
    /// Indica si la métrica térmica es física o heurística.
    pub is_virtual_thermal_reading: bool,
}

pub struct HardwareMonitor;

impl HardwareMonitor {
    /**
     * Captura instantánea de las constantes vitales del hardware.
     *
     * # Logic:
     * Orquesta la lectura de múltiples estratos del sistema operativo
     * para componer el reporte biométrico del nodo.
     */
    #[must_use]
    pub fn capture_instantaneous_metrics() -> NodeHardwareMetrics {
        let current_load_average = Self::read_system_load_average();
        let (calculated_temperature, is_virtual) = Self::read_thermal_status_adaptive(current_load_average);

        NodeHardwareMetrics {
            cpu_frequency_megahertz: Self::read_cpu_frequency_megahertz(),
            system_load_average: current_load_average,
            core_temperature_celsius: calculated_temperature,
            memory_utilization_bytes: Self::calculate_memory_utilization_bytes(),
            is_virtual_thermal_reading: is_virtual,
        }
    }

    /**
     * Lee la frecuencia actual del escalador de la CPU.
     * Fallback: Retorna 0 si el controlador cpufreq está deshabilitado.
     */
    fn read_cpu_frequency_megahertz() -> u32 {
        let frequency_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq";
        fs::read_to_string(frequency_path)
            .unwrap_or_else(|_| "0".to_string())
            .trim()
            .parse::<u32>()
            .map(|khz| khz / 1000) // Conversión de kHz a MHz
            .unwrap_or(0)
    }

    /**
     * Extrae el promedio de carga del sistema (Load Avg 1min) desde /proc/loadavg.
     */
    fn read_system_load_average() -> f32 {
        fs::read_to_string("/proc/loadavg")
            .unwrap_or_default()
            .split_whitespace()
            .next()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(0.0)
    }

    /**
     * Determina el estado térmico mediante sensores físicos o estimación por carga.
     *
     * # Logic:
     * Si los descriptores 'thermal_zone' o 'hwmon' no están disponibles (común en VMs),
     * aplica una fórmula lineal: Base(40.0°C) + (Carga * 20.0°C), limitada a 90.0°C.
     */
    fn read_thermal_status_adaptive(current_load: f32) -> (f32, bool) {
        // Estrato 1: Ruta estándar del kernel
        let primary_thermal_path = "/sys/class/thermal/thermal_zone0/temp";
        if let Ok(content) = fs::read_to_string(primary_thermal_path) {
            if let Ok(parsed_thermal_value) = content.trim().parse::<f32>() {
                if parsed_thermal_value > 0.0 {
                    return (parsed_thermal_value / 1000.0, false);
                }
            }
        }

        // Estrato 2: Interfaz hwmon (Alternativa en algunos hipervisores)
        let secondary_hwmon_path = "/sys/class/hwmon/hwmon0/temp1_input";
        if let Ok(content) = fs::read_to_string(secondary_hwmon_path) {
            if let Ok(parsed_thermal_value) = content.trim().parse::<f32>() {
                if parsed_thermal_value > 0.0 {
                    return (parsed_thermal_value / 1000.0, false);
                }
            }
        }

        // Estrato 3: Heurística Táctica (Fallback Virtual)
        let synthetic_temperature = 40.0 + (current_load * 20.0).min(50.0);
        (synthetic_temperature, true)
    }

    /**
     * Calcula el consumo de RAM analizando los segmentos de /proc/meminfo.
     *
     * # Logic:
     * Implementa un algoritmo de sustracción diferencial (Total - Disponible)
     * para obtener una métrica real de ocupación de la aplicación.
     */
    fn calculate_memory_utilization_bytes() -> u64 {
        if let Ok(meminfo_content) = fs::read_to_string("/proc/meminfo") {
            let mut memory_total_kb = 0u64;
            let mut memory_available_kb = 0u64;

            for line in meminfo_content.lines() {
                if line.starts_with("MemTotal:") {
                    memory_total_kb = Self::parse_kilobyte_value(line);
                }
                if line.starts_with("MemAvailable:") {
                    memory_available_kb = Self::parse_kilobyte_value(line);
                }
            }

            // Fallback para kernels antiguos: Si no hay MemAvailable, estimamos
            // mediante MemTotal (aproximación conservadora)
            if memory_available_kb == 0 {
                return (memory_total_kb / 2) * 1024;
            }

            return memory_total_kb.saturating_sub(memory_available_kb) * 1024;
        }
        0
    }

    /**
     * Helper de extracción de valores numéricos de cadenas procfs.
     */
    fn parse_kilobyte_value(line: &str) -> u64 {
        line.split_whitespace()
            .nth(1)
            .and_then(|val| val.parse::<u64>().ok())
            .unwrap_or(0)
    }
}
