import { client as defaultJupiterApiClient } from "../generated/jupiter/client.gen";
import {
  createClient as createGeneratedJupiterApiClient,
  type Config as JupiterApiClientConfig,
} from "../generated/jupiter/client";
import { client as defaultJupiterTxApiClient } from "../generated/tx/client.gen";
import {
  createClient as createGeneratedJupiterTxApiClient,
  type Config as JupiterTxApiClientConfig,
} from "../generated/tx/client";

export const createJupiterApiClient = (config: JupiterApiClientConfig = {}) =>
  createGeneratedJupiterApiClient({
    ...defaultJupiterApiClient.getConfig(),
    ...config,
  });

export const createJupiterTxApiClient = (config: JupiterTxApiClientConfig = {}) =>
  createGeneratedJupiterTxApiClient({
    ...defaultJupiterTxApiClient.getConfig(),
    ...config,
  });

export type { JupiterApiClientConfig, JupiterTxApiClientConfig };
