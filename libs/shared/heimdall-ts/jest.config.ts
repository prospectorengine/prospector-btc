/* eslint-disable */
export default {
  displayName: 'heimdall-ts',
  preset: '../../../jest.preset.js',
  testEnvironment: 'node', // ✅ Sello de consistencia
  transform: {
    '^.+\\.[tj]s$': ['ts-jest', { tsconfig: '<rootDir>/tsconfig.spec.json' }],
  },
  moduleFileExtensions: ['ts', 'js', 'html'],
  coverageDirectory: '../../../coverage/libs/shared/heimdall-ts',
};
