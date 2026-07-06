export interface CardPoolConfig {
  name: string;
  type: string;
}

export type ResourceSource = "github" | "gitee";
export type LogLevel = "error" | "warn" | "info" | "debug" | "trace";

export interface AppConfig {
  gameRootDir: string | null;
  gameLogFileRelativePath: string;
  resourceSource: ResourceSource;
  dataDir: string | null;
  assetsDir: string | null;
  logLevel: LogLevel;
  skipFirstSSR: boolean;
  baseSsrIds: string[];
  cardPools: CardPoolConfig[];
}

export interface AppConfigState {
  config: AppConfig;
  configFilePath: string;
  resolvedDataDir: string;
  resolvedAssetsDir: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}
