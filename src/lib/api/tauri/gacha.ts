import { invokeCommand } from "$lib/api/tauri/core";
import type {
  AnalyzeLocalPoolResponse,
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

export function parseGachaUrl(url: string) {
  return invokeCommand<ParseGachaUrlResponse>("parse_gacha_url", {
    request: { url },
  });
}
