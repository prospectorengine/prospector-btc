/**
 * APARATO: UI-KIT JEST CONFIG
 * RESPONSABILIDAD: Configuración del motor de pruebas del Design System.
 */
export default {
  displayName: 'ui-kit',
  preset: '../../../jest.preset.js',
  transform: {
    '^(?!.*\\.(js|jsx|ts|tsx|css|json)$)': '@nx/react/plugins/jest',
    '^.+\\.[tj]sx?$': ['ts-jest', { tsconfig: '<rootDir>/tsconfig.spec.json' }],
  },
  moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx'],
  coverageDirectory: '../../../coverage/libs/shared/ui-kit',
  testEnvironment: 'jsdom',
};
