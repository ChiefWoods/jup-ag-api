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
import { client as defaultJupiterPerpsApiClient } from "../generated/perps/client.gen";
import {
  createClient as createGeneratedJupiterPerpsApiClient,
  type Config as JupiterPerpsApiClientConfig,
} from "../generated/perps/client";
import { client as defaultJupiterDatapiClient } from "../generated/datapi/client.gen";
import {
  createClient as createGeneratedJupiterDatapiClient,
  type Config as JupiterDatapiClientConfig,
} from "../generated/datapi/client";

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

export const createJupiterPerpsApiClient = (config: JupiterPerpsApiClientConfig = {}) =>
  createGeneratedJupiterPerpsApiClient({
    ...defaultJupiterPerpsApiClient.getConfig(),
    ...config,
  });

export const createJupiterDatapiClient = (config: JupiterDatapiClientConfig = {}) =>
  createGeneratedJupiterDatapiClient({
    ...defaultJupiterDatapiClient.getConfig(),
    ...config,
  });

export type {
  JupiterApiClientConfig,
  JupiterDatapiClientConfig,
  JupiterPerpsApiClientConfig,
  JupiterTxApiClientConfig,
};
