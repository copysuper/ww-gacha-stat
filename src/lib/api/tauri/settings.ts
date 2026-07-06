import { invokeCommand } from "$lib/api/tauri/core";
import type { AppConfig, AppConfigState } from "$lib/types/dto";

export function getAppConfig() {
  return invokeCommand<AppConfigState>("get_app_config");
}

export function updateAppConfig(config: AppConfig) {
  return invokeCommand<AppConfigState>("update_app_config", {
    request: { config },
  });
}
