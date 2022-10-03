#!/bin/bash

currency="USD"
days=0
chains=( "avalanche-2" "ethereum" "moonbeam" "polkadot" "matic-network" "agoric" "aioz-network" "akash-network" "markhor-meta" "assetmantle" "axelar" "band-protocol" "bitsong" "chihuahua" "cosmos" "crescent-network" "crypto-com-chain" "desmos" "evmos" "graviton" "injective-protocol" "iris-network" "juno-network" "kava" "kujira" "lum-network" "osmosis" "persistence" "regen" "secret" "sifchain" "stargaze" "stride" "tgrade" "umee" )

temp="temp.json"
pricesList="pricesList.txt"
pricesListTemp="pricesListTemp.txt"
finalPrices="finalPrices.txt"
prices=()

for chain in ${chains[@]}; do
    echo $chain
    curl -X 'GET' \
        'https://pro-api.coingecko.com/api/v3/coins/'$chain'/market_chart?vs_currency='$currency'&days='$days'&x_cg_pro_api_key=CG-Fz2vsJvu7rwiL5rJhrbeN5HW' \
        -H 'accept: application/json' &> /dev/null > $temp
    price=($( jq -r '.prices[][1]' $temp ))
    prices+=( $price )
done

echo "[" > $pricesList

for f in "${prices[@]}"; do
  echo '"' >> $pricesList
  priceSubstring="${f:0:11}"
  echo $priceSubstring >> $pricesList
  echo '"' >> $pricesList
  echo "," >> $pricesList
done

tr -d "\n" < $pricesList > $pricesListTemp
finally=$( cat $pricesListTemp | rev | cut -c2- | rev )
echo $finally > $pricesList
echo "]" >> $pricesList

tr -d "\n" < $pricesList > $finalPrices
gsed -i 's/\.//g' $finalPrices