// pages/_app.tsx

import '../styles/globals.css';
import type { AppProps } from 'next/app';
import { WalletProvider } from '@cosmos-kit/react';
import { ChakraProvider } from '@chakra-ui/react';
import { defaultTheme } from '../config';
import { wallets } from '@cosmos-kit/keplr';
import { chains, assets } from 'chain-registry';
import { getSigningCosmosClientOptions } from 'stargaze-zone';
import { GasPrice } from '@cosmjs/stargate';
import { SignerOptions } from '@cosmos-kit/core';
import { Chain } from '@chain-registry/types';

import { chain as testnetChain, assetList as testnetAssets } from '../config/testnet';

function CreateCosmosApp({ Component, pageProps }: AppProps) {
  const signerOptions: SignerOptions = {
    stargate: (_chain: Chain) => {
      return getSigningCosmosClientOptions();
    },
    cosmwasm: (chain: Chain) => {
      switch (chain.chain_name) {
        case 'stargaze':
        case 'stargazetestnet':
          return {
            gasPrice: GasPrice.fromString('0.0025ustars')
          };
      }
    }
  };

  return (
    <ChakraProvider theme={defaultTheme}>
      <WalletProvider
        chains={[...chains, testnetChain]}
        assetLists={[...assets, testnetAssets]}
        wallets={wallets}
        signerOptions={signerOptions}
        endpointOptions={{
          stargaze: {
            rpc: ['https://rpc.elgafar-1.stargaze-apis.com']
          }
        }}
      >
        <Component {...pageProps} />
      </WalletProvider>
    </ChakraProvider>
  );
}

export default CreateCosmosApp;
