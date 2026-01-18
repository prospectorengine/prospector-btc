/**
 * APARATO: API-CLIENT JEST CONFIG (V2.0 - ALIGNED)
 * RESPONSABILIDAD: Certificación de la capa de transporte L4.
 */
export default {
  displayName: 'api-client', // ✅ REPARADO: Paridad con el Alias
  preset: '../../../jest.preset.js',
  testEnvironment: 'node',
  transform: {
    '^.+\\.[tj]s$': ['ts-jest', { tsconfig: '<rootDir>/tsconfig.spec.json' }],
  },
  moduleFileExtensions: ['ts', 'js', 'html'],
  coverageDirectory: '../../../coverage/libs/infra/api-client-ts',
};
