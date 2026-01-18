{
  "name": "@prospector-btc/source",
  "version": "16.1.1",
  "private": true,
  "packageManager": "pnpm@10.0.0",
  "engines": {
    "node": "20.19.0",
    "pnpm": ">=10.0.0"
  },
    "scripts": {
    "postinstall": "husky",
    "i18n:generate": "cross-env TS_NODE_COMPILER_OPTIONS=\"{\\\"module\\\":\\\"commonjs\\\",\\\"esModuleInterop\\\":true}\" ts-node --project tsconfig.base.json apps/web-dashboard/tools/scripts/generate-i18n.ts",
    "test:all": "pnpm i18n:generate && pnpm exec nx run-many -t test --all && cargo test --workspace --release -- --nocapture",
    "build:web": "pnpm i18n:generate && pnpm exec nx build web-dashboard --prod"
  },
  
  },
  "devDependencies": {

  },
  "dependencies": {

  }

}
