import { invokeCommand } from "$lib/api/tauri/core";
import type {
  AnalyzeLocalPoolResponse,
  LoadCachedGachaParamsResponse,
  MergeLocalPoolResponse,
  ParseGachaUrlResponse,
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
