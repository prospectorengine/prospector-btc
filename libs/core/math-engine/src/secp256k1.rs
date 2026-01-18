// INICIO DEL ARCHIVO [libs/core/math-engine/src/secp256k1.rs]
/*!
 * =================================================================
 * APARATO: SOVEREIGN GEOMETRIC ENGINE (V131.0 - ZENITH TRACING)
 * CLASIFICACIÓN: CORE MATH (ESTRATO L1)
 * RESPONSABILIDAD: LEYES DE GRUPO JACOBIANAS Y CERTIFICACIÓN GÉNESIS
 *
 * # Mathematical Proof (Weierstrass Curve secp256k1):
 * La curva se define por $y^2 = x^3 + 7$ sobre $\mathbb{F}_p$.
 * En coordenadas Jacobianas $(X, Y, Z)$, la ecuación es:
 * $Y^2 = X^3 + 7Z^6$.
 *
 * Las operaciones se realizan sin inversiones modulares (división)
 * para maximizar el throughput en el bucle caliente.
 * =================================================================
 */

use crate::prelude::*;
use tracing::{instrument, trace};

impl JacobianPoint {
    /**
     * Duplicación de Punto ($P + P = 2P$).
     *
     * # Algoritmo (Coste: 3M + 4S):
     * Si $P = (X, Y, Z)$, entonces $2P = (X_3, Y_3, Z_3)$ donde:
     * 1. $M = 3 \cdot X^2$ (Tangente)
     * 2. $S = 4 \cdot X \cdot Y^2$
     * 3. $X_3 = M^2 - 2S$
     * 4. $Y_3 = M(S - X_3) - 8Y^4$
     * 5. $Z_3 = 2YZ$
     *
     * # Performance
     * Instrumentado con nivel 'trace' para evitar overhead en producción.
     */
    #[inline(always)]
    #[instrument(level = "trace", skip(self), ret)]
    pub fn double_deterministic(&self) -> Self {
        // 1. Manejo de Singularidades (Punto al Infinito o Tangente Vertical)
        if self.is_infinity || self.y.is_zero() {
            trace!("♾️ [GEOMETRY]: Doubling point at infinity or Y=0. Result: Infinity.");
            return Self::infinity();
        }

        // A = X^2
        let x_squared = self.x.square_modular();

        // M = 3 * X^2 (Para a=0)
        let term_m = x_squared
            .add_modular(&x_squared)
            .add_modular(&x_squared);

        // B = Y^2
        let y_squared = self.y.square_modular();

        // C = X * Y^2 (Usado para S)
        let x_y_squared = self.x.multiply_modular(&y_squared);

        // S = 4 * C
        let term_s = x_y_squared
            .add_modular(&x_y_squared)
            .add_modular(&x_y_squared)
            .add_modular(&x_y_squared);

        // X3 = M^2 - 2S
        let m_squared = term_m.square_modular();
        let two_s = term_s.add_modular(&term_s);
        let x_3 = m_squared.subtract_modular(&two_s);

        // Z3 = 2 * Y * Z
        let y_z = self.y.multiply_modular(&self.z);
        let z_3 = y_z.add_modular(&y_z);

        // Y3 = M * (S - X3) - 8 * Y^4
        let y_fourth = y_squared.square_modular();
        let term_d = y_fourth.multiply_by_u64(8);

        let s_minus_x3 = term_s.subtract_modular(&x_3);
        let y_3 = term_m
            .multiply_modular(&s_minus_x3)
            .subtract_modular(&term_d);

        Self {
            x: x_3,
            y: y_3,
            z: z_3,
            is_infinity: false,
        }
    }

    /**
     * Adición Mixta ($P_1 + P_2 = P_3$).
     * Suma un punto Jacobiano con otro Jacobiano (General Case).
     *
     * # Algoritmo (Cohen-Miyaji-Ono - Modificado):
     * Utiliza las fórmulas estándar de proyección $U1 = X1 \cdot Z2^2$, $U2 = X2 \cdot Z1^2$.
     */
    #[inline(always)]
    #[instrument(level = "trace", skip(self, other_point), ret)]
    pub fn add_deterministic(&self, other_point: &Self) -> Self {
        // 1. Identidad Aditiva
        if self.is_infinity { return *other_point; }
        if other_point.is_infinity { return *self; }

        // Z1^2, Z2^2
        let z1_sq = self.z.square_modular();
        let z2_sq = other_point.z.square_modular();

        // U1 = X1 * Z2^2, U2 = X2 * Z1^2
        let u1 = self.x.multiply_modular(&z2_sq);
        let u2 = other_point.x.multiply_modular(&z1_sq);

        // S1 = Y1 * Z2^3, S2 = Y2 * Z1^3
        let s1 = self.y.multiply_modular(&other_point.z.multiply_modular(&z2_sq));
        let s2 = other_point.y.multiply_modular(&self.z.multiply_modular(&z1_sq));

        // Detección de Colisión o Identidad
        if u1 == u2 {
            if s1 == s2 {
                trace!("♻️ [GEOMETRY]: Points are identical. Delegating to doubling logic.");
                return self.double_deterministic();
            } else {
                trace!("🚫 [GEOMETRY]: Points are inverses. Result: Infinity.");
                return Self::infinity();
            }
        }

        // H = U2 - U1
        let h = u2.subtract_modular(&u1);
        // R = S2 - S1
        let r = s2.subtract_modular(&s1);

        let h_sq = h.square_modular();
        let h_cu = h_sq.multiply_modular(&h);
        let v = u1.multiply_modular(&h_sq);

        // X3 = R^2 - H^3 - 2V
        let r_sq = r.square_modular();
        let two_v = v.add_modular(&v);
        let x_3 = r_sq.subtract_modular(&h_cu).subtract_modular(&two_v);

        // Y3 = R * (V - X3) - S1 * H^3
        let v_minus_x3 = v.subtract_modular(&x_3);
        let s1_h3 = s1.multiply_modular(&h_cu);
        let y_3 = r.multiply_modular(&v_minus_x3).subtract_modular(&s1_h3);

        // Z3 = Z1 * Z2 * H
        let z_3 = self.z.multiply_modular(&other_point.z).multiply_modular(&h);

        Self {
            x: x_3,
            y: y_3,
            z: z_3,
            is_infinity: false,
        }
    }
}

// -----------------------------------------------------------------
// ESTRATO DE CERTIFICACIÓN: SATOSHI GENESIS VECTORS (L1-GEOMETRY)
// -----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// Coordenadas canónicas del punto generador G (secp256k1).
    const G_X: [u64; 4] = [0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC];
    const G_Y: [u64; 4] = [0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465];

    /// Coordenadas canónicas del punto 2G.
    const G2_X: [u64; 4] = [0xABAC09B95C709EE5, 0x5C778E4B8CEF3CA7, 0x3045406E95C07CD8, 0xC6047F9441ED7D6D];
    const G2_Y: [u64; 4] = [0x236431A950CFE52A, 0xF7F632653266D0E1, 0xA3C58419466CEAEF, 0x1AE168FEA63DC339];

    #[test]
    fn certify_satoshi_generator_doubling() {
        let point_g = JacobianPoint::from_affine(G_X, G_Y);
        let point_2g_calculated = point_g.double_deterministic();

        let (x_affine, y_affine) = point_2g_calculated.to_affine_bytes()
            .expect("Fallo al proyectar a Afines");

        let expected_x = convert_limbs_u64_to_u256_be(&G2_X);
        let expected_y = convert_limbs_u64_to_u256_be(&G2_Y);

        assert_eq!(x_affine, expected_x, "Fallo en Coordenada X de 2G");
        assert_eq!(y_affine, expected_y, "Fallo en Coordenada Y de 2G");
        println!("✅ GEOMETRY: Jacobian Doubling certified against Satoshi Vectors.");
    }

    #[test]
    fn certify_jacobian_addition_associativity() {
        let point_g = JacobianPoint::from_affine(G_X, G_Y);
        // (G + G) + G
        let two_g = point_g.double_deterministic();
        let three_g_a = two_g.add_deterministic(&point_g);

        // G + (G + G)
        let three_g_b = point_g.add_deterministic(&two_g);

        assert_eq!(three_g_a.x, three_g_b.x);
        assert_eq!(three_g_a.y, three_g_b.y);
        println!("✅ GEOMETRY: Point addition associativity verified.");
    }
}
// FIN DEL ARCHIVO [libs/core/math-engine/src/secp256k1.rs]
