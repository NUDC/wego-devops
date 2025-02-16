/// <reference types="vite/client" />

interface Window {
  readonly PKG: Record<string, string>;
}

interface ImportMetaEnv {
  /** 网站标题 */
  readonly VITE_TITLE: string;
  /** 当前web端口 */
  readonly VITE_CLI_PORT: string;
  /** 服务端httpapi 地址 */
  readonly VITE_SERVER_HTTPAPI: string;
  readonly VITE_PROXY_WS_API: String;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
