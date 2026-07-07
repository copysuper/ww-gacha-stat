import { invokeCommand } from "$lib/api/tauri/core";
import type {
  AnalyzeLocalPoolResponse,
  ExtractLatestGachaUrlResponse,
  LoadCachedGachaParamsResponse,
  MergeLocalPoolResponse,
  ParseGachaUrlResponse,
  RefreshGachaDataResponse,
} from "$lib/types/dto";

export function analyzeLocalPool(filePath: string) {
  return invokeCommand<AnalyzeLocalPoolResponse>("analyze_local_pool", {
    request: { filePath },
  });
}

export function mergeLocalPool(
  newFilePath: string,
  outputFilePath: string,
  oldFilePath?: string,
) {
  return invokeCommand<MergeLocalPoolResponse>("merge_local_pool", {
    request: {
      oldFilePath: oldFilePath?.trim() ? oldFilePath : null,
      newFilePath,
      outputFilePath,
    },
  });
}

export function parseGachaUrl(url: string, saveToCache = false) {
  return invokeCommand<ParseGachaUrlResponse>("parse_gacha_url", {
    request: { url, saveToCache },
  });
}

export function loadCachedGachaParams(playerId: string) {
  return invokeCommand<LoadCachedGachaParamsResponse>(
    "load_cached_gacha_params",
    {
      request: { playerId },
    },
  );
}

export function refreshGachaData(playerId: string) {
  return invokeCommand<RefreshGachaDataResponse>("refresh_gacha_data", {
    request: { playerId },
  });
}

export function extractLatestGachaUrl(logFilePath?: string) {
  return invokeCommand<ExtractLatestGachaUrlResponse>(
    "extract_latest_gacha_url",
    {
      request: { logFilePath: logFilePath?.trim() ? logFilePath : null },
    },
  );
}
