import { useEffect, useState } from 'react';
import { useWallet } from '@cosmos-kit/react';
import { StdFee } from '@cosmjs/amino';
import { assets } from '../config/assets';
import { AssetList, Asset } from '@chain-registry/types';
import { SigningStargateClient } from '@cosmjs/stargate';
import BigNumber from 'bignumber.js';
import {
  Box,
  Divider,
  Grid,
  Heading,
  Text,
  Stack,
  Container,
  Link,
  Button,
  Flex,
  Icon,
  Input,
  useColorMode,
  useColorModeValue
} from '@chakra-ui/react';
import { BsFillMoonStarsFill, BsFillSunFill } from 'react-icons/bs';
import { dependencies, products } from '../config';
import { HackCw20QueryClient } from '../codegen/HackCw20.client';
import { chainNames } from '../static/coingeckoMap';
import {useInterval} from '../utils/utils';
import {  Dependency, WalletSection } from '../components';
import { cosmos } from 'stargaze-zone';
import Head from 'next/head';

const library = {
  title: 'StargazeJS',
  text: 'Typescript libraries for the Stargaze ecosystem',
  href: 'https://github.com/cosmology-tech/stargaze-zone'
};

const chainName = 'stargazetestnet';

const filteredAssets = assets.filter((asset) => asset.chain_name !== 'cryptoorgchain' && chainNames.includes(asset.chain_name));

const chainassets: AssetList = assets.find(
  (chain) => chain.chain_name === chainName
) as AssetList;
const coin: Asset = chainassets.assets.find(
  (asset) => asset.base === 'ustars'
) as Asset;


const sendTokens = (
  getStargateClient: () => Promise<SigningStargateClient>,
  setResp: () => any,
  address: string
) => {
  return async () => {
    const stargateClient = await getStargateClient();
    if (!stargateClient || !address) {
      console.error('stargateClient undefined or address undefined.');
      return;
    }

    const { send } = cosmos.bank.v1beta1.MessageComposer.withTypeUrl;

    const msg = send({
      amount: [
        {
          denom: coin.base,
          amount: '1000'
        }
      ],
      toAddress: address,
      fromAddress: address
    });

    const fee: StdFee = {
      amount: [
        {
          denom: coin.base,
          amount: '864'
        }
      ],
      gas: '86364'
    };
    const response = await stargateClient.signAndBroadcast(address, [msg], fee);
    setResp(JSON.stringify(response, null, 2));
  };
};

export default function Home() {
  const { colorMode, toggleColorMode } = useColorMode();

  const {
    getStargateClient,
    getCosmWasmClient,
    address,
    setCurrentChain,
    currentWallet,
    walletStatus
  } = useWallet();

  useEffect(() => {
    setCurrentChain(chainName);
  }, [chainName]);

   const [cw20Client, setCw20Client] = useState<HackCw20QueryClient | null>(
    null
  );


    useEffect(() => {
    getCosmWasmClient().then((cosmwasmClient) => {
      if (!cosmwasmClient || !address) {
        console.error('stargateClient undefined or address undefined.');
        return;
      }

      setCw20Client(
        new HackCw20QueryClient(
          cosmwasmClient,
          'stars10w8tguqxpgyfn38wqlldces6amg3vqt662rjudf2lwfz5rag9ykq0pem3g'
        )
      );
    });
  }, [address, getCosmWasmClient]);
    console.log(cw20Client?.getHistoryOfRounds())

    const getAllValues = () => {
      cw20Client?.queryAllValues().then(val =>  {
        const splitVal = val[0].split(':')[1]
        console.log('split val', splitVal)
        setPrices(JSON.parse(splitVal))
      });
    }

  const [prices, setPrices] = useState([])
  useEffect(() => {
    if (cw20Client && address) {
      getAllValues()
    }
  }, [cw20Client, address]);

   useInterval(() => {
    if (cw20Client && address) {
      getAllValues()
    }
  }, 3000 * 1);

  const [balance, setBalance] = useState(new BigNumber(0));
  const [resp, setResp] = useState('');
  const getBalance = async () => {
    if (!address) {
      setBalance(new BigNumber(0));
      return;
    }

    let rpcEndpoint = await currentWallet?.getRpcEndpoint();

    if (!rpcEndpoint) {
      console.log('no rpc endpoint — using a fallback');
      rpcEndpoint = `https://rpc.cosmos.directory/${chainName}`;
    }

    // get RPC client
    const client = await cosmos.ClientFactory.createRPCQueryClient({
      rpcEndpoint
    });

    // fetch balance
    const balance = await client.cosmos.bank.v1beta1.balance({
      address,
      denom: chainassets?.assets[0].base as string
    });

    // Get the display exponent
    // we can get the exponent from chain registry asset denom_units
    const exp = coin.denom_units.find((unit) => unit.denom === coin.display)
      ?.exponent as number;

    // show balance in display values by exponentiating it
    const a = new BigNumber(balance.balance.amount);
    const amount = a.multipliedBy(10 ** -exp);
    setBalance(amount);
  };

  const color = useColorModeValue('primary.500', 'primary.200');

  const [filteredAssetsBySearch, setFilteredAssets] = useState(filteredAssets)
  const [search, setSearch] = useState('')

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearch(event.target.value)
     setFilteredAssets(filteredAssets.filter((asset) => {
      if (!asset.chain_name) {
        return false
      }

      return asset.chain_name.toLowerCase().includes(event.target.value.toLowerCase())
  }))
  }

  return (
    <Container maxW="5xl" py={10}>
      <Head>
        <title>Whoa, Oracle</title>
        <meta name="description" content="Generated by create cosmos app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Flex alignItems="center" justifyContent="end" mb={4}>
          <WalletSection chainName={chainName} />
        <Button variant="outline" px={0} onClick={toggleColorMode}>
          <Icon
            as={colorMode === 'light' ? BsFillMoonStarsFill : BsFillSunFill}
          />
        </Button>
      </Flex>
      <Box textAlign="center">
       <Heading
          as="h1"
          fontSize={{ base: '3xl', sm: '4xl', md: '5xl' }}
          fontWeight="extrabold"
          mb={3}
        >
          Oracle Standard Dataset
        </Heading>
          <Heading
          fontSize={{ base: 'lg', sm: 'md', md: 'md' }}
        >
          <Text  as="span">A reference price data powered by Osmosis</Text>
        </Heading>
      </Box>

      <Box mb={8}>
      </Box>

      <Input
        placeholder='Search protocols'
        value={search}
        onChange={handleChange}
      />

      <Box mb={6}>
      </Box>

      <Text fontSize='lg' fontWeight="semibold">Cryptocurrencies</Text>

      <Box mb={3}>
      </Box>
      <Box mb={3}>
        <Divider />
      </Box>

      <Grid templateColumns={{ md: 'repeat(auto-fill, 22%)' }} justifyContent="center" gap={8} mb={20}>
        {filteredAssetsBySearch.map((asset, index) => (
          <Dependency key={asset.chain_name + index.toString()} {...asset.assets[0]} { ...{price: prices[index]}}></Dependency>
        ))}
      </Grid>
      <Box mb={3}>
        <Divider />
      </Box>
      <Stack
        isInline={true}
        spacing={1}
        justifyContent="center"
        opacity={0.5}
        fontSize="sm"
      >
        <Text>Built with</Text>
        <Link
          href="https://cosmology.tech/"
          target="_blank"
          rel="noopener noreferrer"
        >
          Cosmology
        </Link>
      </Stack>
    </Container>
  );
}
