/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_APP_NAME: string;
  readonly VITE_DEFAULT_COVER: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
