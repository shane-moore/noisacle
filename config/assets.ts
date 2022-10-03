import { assets as chainRegistryAssets } from "chain-registry";
import { assetList as testnetAssets } from "./testnet";

export const assets = [...chainRegistryAssets, testnetAssets];
