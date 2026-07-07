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

export interface HitData {
  id: number;
  name: string;
  count: number;
  event: boolean;
  date: string;
}

export interface PoolRankSummary {
  count: number;
  rate: number;
  avg: number;
  min: number;
  max: number;
  currentPity: number;
}

export interface PoolAnalysisSummary {
  poolName: string;
  isEmpty: boolean;
  totalCount: number;
  startDate: string | null;
  endDate: string | null;
  ssr: PoolRankSummary;
  sr: PoolRankSummary;
  r: PoolRankSummary;
  ssrEventCount: number;
  ssrPermanentCount: number;
  latestSsr: HitData | null;
  latestSr: HitData | null;
}

export interface AnalysisData {
  isEmpty: boolean;
  poolName: string;
  totalCount: number;
  noUpSsrCount: number;
  noUpSrCount: number;
  noUpRCount: number;
  ssrCount: number;
  srCount: number;
  rCount: number;
  ssrAvg: number;
  ssrMin: number;
  ssrMax: number;
  srAvg: number;
  srMin: number;
  srMax: number;
  rAvg: number;
  rMin: number;
  rMax: number;
  startDate: string | null;
  endDate: string | null;
  ssrDataList: HitData[];
  srDataList: HitData[];
  rDataList: HitData[];
}

export interface AnalyzeLocalPoolResponse {
  filePath: string;
  analysisList: AnalysisData[];
  summaryList: PoolAnalysisSummary[];
}

export interface PoolMergeSummary {
  poolName: string;
  oldCount: number;
  newCount: number;
  mergedCount: number;
  appendedOldCount: number;
}

export interface PoolMergeResult {
  summaries: PoolMergeSummary[];
  totalOldCount: number;
  totalNewCount: number;
  totalMergedCount: number;
  totalAppendedOldCount: number;
}

export interface MergeLocalPoolResponse {
  oldFilePath: string | null;
  newFilePath: string;
  outputFilePath: string;
  mergeResult: PoolMergeResult;
  analysisList: AnalysisData[];
  summaryList: PoolAnalysisSummary[];
}

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}
