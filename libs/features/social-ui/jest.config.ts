/**
 * APARATO: SOCIAL-UI JEST CONFIG
 * RESPONSABILIDAD: Pruebas de integración para el Community Hub.
 */
export default {
  displayName: 'ui-social',
  preset: '../../../jest.preset.js',
  transform: {
    '^(?!.*\\.(js|jsx|ts|tsx|css|json)$)': '@nx/react/plugins/jest',
    '^.+\\.[tj]sx?$': ['ts-jest', { tsconfig: '<rootDir>/tsconfig.spec.json' }],
  },
  moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx'],
  coverageDirectory: '../../../coverage/libs/features/social-ui',
  testEnvironment: 'jsdom',
};
