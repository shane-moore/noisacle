import {
  Box,
  Stack,
  Text,
  useColorModeValue
} from '@chakra-ui/react';
import Image from 'next/image'
import { AssetWithJpeg} from './types';



export const Dependency = ({  symbol, logo_URIs, price  }: AssetWithJpeg) => {
  const imgSrc = logo_URIs ? logo_URIs.png || logo_URIs.svg || logo_URIs.jpeg ||  '' : ''

  return (
      <Stack
        isInline={true}
        key={symbol}
        spacing={3}
        h="full"
        p={4}
        justifyContent="center"
        borderRadius="md"
        border="1px solid"
        borderColor={useColorModeValue('blackAlpha.200', 'whiteAlpha.100')}
        _hover={{
          boxShadow: useColorModeValue(
            '0 2px 5px #ccc',
            '0 1px 3px #727272, 0 2px 12px -2px #2f2f2f'
          )
        }}
      >
        <Box color={useColorModeValue('primary.500', 'primary.200')}>
        </Box>
        <div style={{display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '5px'}}>
          <div style={{display:'flex', alignItems: 'center', gap: '12px'}}>
            <Image src={imgSrc} alt={symbol} width="30" height="30" />
            <Text fontSize="xl" fontWeight="semibold" textAlign="center">{symbol} </Text>
          </div>
          <p style={{fontWeight: 'bold', fontSize: '24px'}}>{price}</p>
        </div>
        {symbol}
      </Stack>
  );
};
