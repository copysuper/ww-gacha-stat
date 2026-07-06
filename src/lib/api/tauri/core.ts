import { invoke } from "@tauri-apps/api/core";

import type { ApiResponse } from "$lib/types/dto";

export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  const response = await invoke<ApiResponse<T>>(command, args);

  if (!response.success || response.data === null) {
    throw new Error(response.error ?? `${command} 执行失败`);
  }

  return response.data;
}
