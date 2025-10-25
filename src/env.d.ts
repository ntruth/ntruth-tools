/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_MOCK_TRANSLATE_API?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
