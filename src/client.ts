import { client as defaultJupiterApiClient } from "../generated/ts/jupiter/client.gen";
import {
  createClient as createGeneratedJupiterApiClient,
  type Config as JupiterApiClientConfig,
} from "../generated/ts/jupiter/client";
import { client as defaultJupiterTxApiClient } from "../generated/ts/tx/client.gen";
import {
  createClient as createGeneratedJupiterTxApiClient,
  type Config as JupiterTxApiClientConfig,
} from "../generated/ts/tx/client";

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
