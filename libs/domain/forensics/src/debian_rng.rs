// INICIO DEL ARCHIVO [libs/domain/forensics/src/debian_rng.rs]
/*!
 * =================================================================
 * APARATO: DEBIAN VULNERABILITY ITERATOR (V35.1 - FIXED)
 * =================================================================
 */

use byteorder::{ByteOrder, LittleEndian};
use prospector_core_math::prelude::*;
// ✅ REPARACIÓN: Tracing ahora disponible vía Cargo.toml
use tracing::{debug, instrument};

pub const DEBIAN_PID_MAX_STRATUM: u32 = 32767;

pub struct DebianForensicIterator {
    current_iteration_pid: u32,
    maximum_target_pid: u32,
}

impl DebianForensicIterator {
    pub fn new(starting_pid: u32, ending_pid: u32) -> Self {
        Self {
            current_iteration_pid: starting_pid,
            maximum_target_pid: ending_pid.min(DEBIAN_PID_MAX_STRATUM),
        }
    }

    #[inline(always)]
    fn synthesize_weak_private_key(process_identifier: u32) -> SafePrivateKey {
        let mut seed_buffer = [0u8; 32];
        LittleEndian::write_u32(&mut seed_buffer[0..4], process_identifier);
        seed_buffer[4..32].fill(0x00);

        SafePrivateKey::from_bytes(&seed_buffer)
            .unwrap_or_else(|_| SafePrivateKey::new_random())
    }
}

impl Iterator for DebianForensicIterator {
    type Item = (String, SafePrivateKey);

    #[instrument(skip(self))]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iteration_pid > self.maximum_target_pid {
            return None;
        }

        let active_pid = self.current_iteration_pid;
        self.current_iteration_pid += 1;

        let private_key_instance = Self::synthesize_weak_private_key(active_pid);
        let metadata_label = format!("forensic_debian_2008:pid_{}", active_pid);

        debug!("🧬 [DEBIAN_RNG]: Synthesized key for PID {}", active_pid);

        Some((metadata_label, private_key_instance))
    }
}
// FIN DEL ARCHIVO [libs/domain/forensics/src/debian_rng.rs]
