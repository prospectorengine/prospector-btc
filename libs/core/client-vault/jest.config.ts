/**
 * APARATO: CRYPTO-VAULT JEST CONFIG (V2.0 - ALIGNED)
 * RESPONSABILIDAD: Certificación de pruebas para el motor AES-GCM.
 */
export default {
  displayName: 'crypto-vault', // ✅ REPARADO: Paridad con el Alias
  preset: '../../../jest.preset.js',
  testEnvironment: 'jsdom', // ✅ REPARADO: Requerido para WebCrypto
  transform: {
    '^.+\\.[tj]s$': ['ts-jest', { tsconfig: '<rootDir>/tsconfig.spec.json' }],
  },
  moduleFileExtensions: ['ts', 'js', 'html'],
  coverageDirectory: '../../../coverage/libs/core/client-vault',
};
