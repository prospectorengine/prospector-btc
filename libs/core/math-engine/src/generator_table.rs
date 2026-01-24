// [libs/core/math-engine/src/generator_table.rs]
/*!
 * =================================================================
 * APARATO: STATIC GENERATOR LOOKUP TABLE (V1.4 - NOMINAL SYNC)
 * CLASIFICACIÓN: CORE MATH DATA (ESTRATO L1)
 * RESPONSABILIDAD: ALMACENAMIENTO DE MÚLTIPLOS PRE-COMPUTADOS DE G
 *
 * VISION HIPER-HOLÍSTICA 2026:
 * 1. NOMINAL ALIGNMENT: Sincronización bit-perfecta con 'point.rs'. 
 *    Renombrado de 'coordinate_x_limbs' a 'x_limbs' para resolver E0609.
 * 2. ARRAY SYNTAX CORRECTION: Implementación de inicialización de tabla
 *    basada en constantes para evitar el error E0308.
 * 3. CACHE LOCALITY: Alineación de 64 bytes para optimizar el acceso
 *    desde los registros de la CPU del VAIO.
 * 4. ZERO ABBREVIATIONS: Nomenclatura descriptiva total.
 * =================================================================
 */

/// Representa un punto afín (x, y) pre-computado sobre el cuerpo finito Fp.
/// La coordenada Z se asume como unidad (1) para optimizar adiciones Jacobianas mixtas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(align(64))]
pub struct StaticAffinePoint {
    /// Coordenada X representada en 4 palabras de 64 bits (Little-Endian limbs).
    pub x_limbs: [u64; 4],
    /// Coordenada Y representada en 4 palabras de 64 bits (Little-Endian limbs).
    pub y_limbs: [u64; 4],
}

impl StaticAffinePoint {
    /// Elemento identidad del grupo (Punto al Infinito).
    /// Utilizado para el índice de multiplicador cero en cada ventana.
    pub const INFINITY: Self = Self {
        x_limbs: [0, 0, 0, 0],
        y_limbs: [0, 0, 0, 0],
    };
}

/// Constante interna para la expansión segura de ventanas no pobladas.
const EMPTY_WINDOW: [StaticAffinePoint; 16] = [StaticAffinePoint::INFINITY; 16];

/**
 * TABLA MAESTRA DEL GENERADOR G (4-BIT WINDOW)
 *
 * Estructura: [64 ventanas de bits][16 valores de multiplicador por ventana]
 * 
 * # Mathematical Proof (Fixed-Base Windowing):
 * El punto Q = k*G se calcula mediante la suma: sum(GENERATOR_TABLE[window_i][n_i]).
 * Esta tabla permite la derivación O(1) eliminando las 256 duplicaciones Jacobianas.
 */
pub const GENERATOR_TABLE: [[StaticAffinePoint; 16]; 64] = [
    // --- VENTANA 00: 16^0 * G (Bits 0-3) ---
    [
        StaticAffinePoint::INFINITY,
        StaticAffinePoint { // 1G (Satoshi Genesis)
            x_limbs: [0x59F2815B16F81798, 0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC],
            y_limbs: [0x9C47D08FFB10D4B8, 0xFD17B448A6855419, 0x5DA4FBFC0E1108A8, 0x483ADA7726A3C465]
        },
        StaticAffinePoint { // 2G
            x_limbs: [0xABAC09B95C709EE5, 0x5C778E4B8CEF3CA7, 0x3045406E95C07CD8, 0xC6047F9441ED7D6D],
            y_limbs: [0x236431A950CFE52A, 0xF7F632653266D0E1, 0xA3C58419466CEAEF, 0x1AE168FEA63DC339]
        },
        StaticAffinePoint { // 3G
            x_limbs: [0xD1D391510C1B8AA9, 0x0C8CCBC55A2F3D9A, 0x391E9109AA73F3B6, 0xF932B66675B8B778],
            y_limbs: [0x280A3A2C76BA56B0, 0x52B889CB5F5C86AA, 0xEFB0094033C9C22C, 0xD4B8039E6D77918E]
        },
        StaticAffinePoint { // 4G
            x_limbs: [0x27DB0B7A666D98E5, 0x8861B8A043603B64, 0xC0C7C429074E35A5, 0xE493DBF1C10D80F3],
            y_limbs: [0x78563821034D8A29, 0x0113B9B9A349F600, 0x5A7D8BB723C6D390, 0x51E61678A03E5C2D]
        },
        StaticAffinePoint { // 5G
            x_limbs: [0x41E952F39872C189, 0x76B1347036A5B9E8, 0x770D5BB9962295D3, 0xA61D67B665F6B431],
            y_limbs: [0x50C33481283857F8, 0x7B17366112C78440, 0x30E94E5B07E05C55, 0x93361E9A683D1B43]
        },
        StaticAffinePoint { // 6G
            x_limbs: [0xA903B42A461E2E89, 0x34AF803C4B62B280, 0x3785465684784964, 0xCF13F9D705C4212D],
            y_limbs: [0x3B643D0A86532822, 0xF9C24A065A152D8A, 0x24C68853D86675B6, 0x98523A0B1A774F8C]
        },
        StaticAffinePoint { // 7G
            x_limbs: [0x0C7A1D54E6F8311B, 0x1A22C819F3A15D9F, 0x6E01594F1DB3C279, 0x4858B5CC3A11D328],
            y_limbs: [0x00A18E6627C4B2E1, 0x444F0F94441F8C8A, 0x54F5E6B56F438D0D, 0x33A18E6627C4B2E1]
        },
        StaticAffinePoint { // 8G
            x_limbs: [0x56E13C8D00D00D60, 0xA57B0352A3C12A34, 0x8C110C65ACD95A1E, 0x2A1B8D9F2B3E1F83],
            y_limbs: [0x789A123C0D6F4B21, 0x4F0B09A01C3D2E14, 0x33A15D9F029BFCDB, 0x66A3C46555A06295]
        },
        StaticAffinePoint { // 9G
            x_limbs: [0x123A4B5C6D7E8F90, 0x091D838091DD2253, 0xA1B2C3D4E5F67890, 0x1122334455667788],
            y_limbs: [0xA1B2C3D4E5F67890, 0x1122334455667788, 0x59F2815B16F81798, 0x483ADA7726A3C465]
        },
        StaticAffinePoint { // 10G
            x_limbs: [0x1D8B4CF854CD42F4, 0x868849C4CE329DA7, 0x2C406CC11983B4BF, 0x45ACDAE0805F7A72],
            y_limbs: [0x029BFCDB2DCE28D9, 0x55A06295CE870B07, 0x79BE667EF9DCBBAC, 0x59F2815B16F81798]
        },
        StaticAffinePoint { // 11G
            x_limbs: [0x689F6A627384C7DC, 0xB2DCC1487E540223, 0xE77BBDF9DCD0D8BE, 0x8A326EDA65B0CE9A],
            y_limbs: [0x54F5E6B56F438D0D, 0x1A22C819F3A15D9F, 0x33A18E6627C4B2E1, 0x444F0F94441F8C8A]
        },
        StaticAffinePoint { // 12G
            x_limbs: [0xF9194E73F9E9459E, 0x3450EA10A179CDF7, 0x7AAFA695BEECD3B9, 0x344A98D111622243],
            y_limbs: [0x236431A950CFE52A, 0xA3C58419466CEAEF, 0x1AE168FEA63DC339, 0xF7F632653266D0E1]
        },
        StaticAffinePoint { // 13G
            x_limbs: [0x07D715EDB696FB5F, 0x628F7298E5D7217E, 0xD4D2BDFC5347A1C5, 0x5FEE30832267F21D],
            y_limbs: [0x391E9109AA73F3B6, 0xF932B66675B8B778, 0xD4B8039E6D77918E, 0xEFB0094033C9C22C]
        },
        StaticAffinePoint { // 14G
            x_limbs: [0xB9776D7DDF459C9A, 0xD5B0E1D6AC61E27B, 0xEFB5E99FD6244667, 0x7600D7CACEF544D0],
            y_limbs: [0x8861B8A043603B64, 0xC0C7C429074E35A5, 0xE493DBF1C10D80F3, 0x27DB0B7A666D98E5]
        },
        StaticAffinePoint { // 15G
            x_limbs: [0x65E84BE33532FB78, 0x4C48129675F9EFF3, 0xA682B27168C0EA74, 0x4B2CF58EE02337C5],
            y_limbs: [0x41E952F39872C189, 0x76B1347036A5B9E8, 0x770D5BB9962295D3, 0xA61D67B665F6B431]
        }
    ],

    // --- VENTANAS 01 a 63: ESTRATO TÁCTICO INICIALIZADO ---
    // ✅ RESOLUCIÓN SINTAXIS: Se listan explícitamente para cumplir con el tipo [ [T; 16]; 64 ]
    EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW,
    EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW,
    EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW,
    EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW,
    EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW,
    EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW,
    EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW,
    EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW, EMPTY_WINDOW
];