/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.17.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { QueryMsg, Addr } from "./HackCw20.types";
export interface HackCw20ReadOnlyInterface {
  contractAddress: string;
  queryIsSelected: ({
    node,
    roundId,
  }: {
    node: Addr;
    roundId: string;
  }) => Promise<QueryIsSelectedResponse>;
  queryAllValues: () => Promise<QueryAllValuesResponse>;
  getHistoryOfRounds: () => Promise<GetHistoryOfRoundsResponse>;
}
export class HackCw20QueryClient implements HackCw20ReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.queryIsSelected = this.queryIsSelected.bind(this);
    this.queryAllValues = this.queryAllValues.bind(this);
    this.getHistoryOfRounds = this.getHistoryOfRounds.bind(this);
  }

  queryIsSelected = async ({
    node,
    roundId,
  }: {
    node: Addr;
    roundId: string;
  }): Promise<QueryIsSelectedResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_is_selected: {
        node,
        round_id: roundId,
      },
    });
  };
  queryAllValues = async (): Promise<QueryAllValuesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_all_values: {},
    });
  };
  getHistoryOfRounds = async (): Promise<GetHistoryOfRoundsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_history_of_rounds: {},
    });
  };
}
