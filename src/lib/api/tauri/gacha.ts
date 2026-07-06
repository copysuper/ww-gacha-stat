import { invokeCommand } from "$lib/api/tauri/core";
import type { AnalyzeLocalPoolResponse } from "$lib/types/dto";

export function analyzeLocalPool(filePath: string) {
  return invokeCommand<AnalyzeLocalPoolResponse>("analyze_local_pool", {
    request: { filePath },
  });
}
