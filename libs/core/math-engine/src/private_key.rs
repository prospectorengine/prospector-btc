// INICIO DEL ARCHIVO [libs/core/math-engine/src/private_key.rs]
// =================================================================
// APARATO: PRIVATE KEY MANAGER (V20.0 - CLONE ENABLED)
// RESPONSABILIDAD: GESTIÓN SEGURA DE ESCALARES SECRETOS
// FIX: Agregado #[derive(Debug, Clone)] para permitir reutilización en estrategias duales.
// =================================================================

use crate::context::global_context;
use crate::errors::MathError;
use secp256k1::rand::rngs::OsRng;
use secp256k1::SecretKey;

/// Wrapper seguro para una clave privada de curva elíptica secp256k1.
/// Garantiza: 0 < k < n
#[derive(Debug, Clone)] // ✅ FIX: Habilitada clonación para estrategias duales
pub struct SafePrivateKey {
    inner: SecretKey,
}

impl SafePrivateKey {
    /// Genera una nueva clave privada utilizando el CSPRNG del sistema.
    pub fn new_random() -> Self {
        let secp = global_context();
        let (secret_key, _) = secp.generate_keypair(&mut OsRng);
        Self { inner: secret_key }
    }

    /// Intenta construir una clave privada a partir de bytes crudos.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MathError> {
        if bytes.len() != 32 {
            return Err(MathError::InvalidLength {
                expected: 32,
                got: bytes.len(),
            });
        }

        let sk = SecretKey::from_slice(bytes).map_err(MathError::EllipticCurveError)?;

        Ok(Self { inner: sk })
    }

    /// Serializa la clave a 32 bytes.
    #[inline]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.inner.secret_bytes()
    }

    /// Referencia al objeto interno (Zero-Copy).
    #[inline(always)]
    pub fn as_inner(&self) -> &SecretKey {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_random_key_entropy() {
        let mut set = HashSet::new();
        for _ in 0..100 {
            let key = SafePrivateKey::new_random();
            assert!(set.insert(key.to_bytes()), "Colisión de entropía detectada");
        }
    }
}
// FIN DEL ARCHIVO [libs/core/math-engine/src/private_key.rs]
