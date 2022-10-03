#!/usr/local/bin/bash
# requires bash 4.0 or later, install via brew install bash

ADDRESS_INDEX=$1

if [ -z "$ADDRESS_INDEX" ]; then
    echo "Usage: oracle.sh <address_index>"
    exit 1
fi

MNEMONIC1="later execute quarter lend shell fish autumn mountain blossom country whisper involve area mouse eight drill expire ball math suit student cable expire sad"
ADDRESS1="stars1gcqwptvwequcxr5x2tvf68g23vph2allxnw9vp"

MNEMONIC2="silly alone fatal guard pass casual snap legal rural flavor slice lecture park deal duty echo street whisper aunt visual crawl cousin dismiss awkward"
ADDRESS2="stars1d7k8m746wz459qjfqzr5j2wecvs8z3czfa8f0d"

MNEMONIC3="gesture step cause umbrella fun army casino repeat speak elevator lamp genuine alert tuna scout creek picture unable inside car clerk drink hunt tray"
ADDRESS3="stars1thttjjks0ey0wmr9jnrrrg68qmqh0t69sh3wj3"

MNEMONIC4="ketchup settle rubber ladder upon whale heavy regular theory praise whip train make fever cabbage grace repair tortoise employ black figure jaguar floor model"
ADDRESS4="stars1jqcfd734yejf0m9cg0a93kt9szyf3kwn3zxt8h"

MNEMONIC5="rural bless input staff outer alter version then empower vast confirm domain vanish nest limit bridge surface client bulk erase sword evidence security party"
ADDRESS5="stars1dtzv68kk09j70xj9gnmmk4sm5vsn67jcsupd8e"

MNEMONIC6="ritual differ tree arm load myth frown begin fun ranch harvest right tenant warm uniform distance resource garbage frost shiver dust head point diary"
ADDRESS6="stars1a3pd99tn5vzx0t8lcm8dfncurxre6f3e9xfu5v"

MNEMONIC7="vibrant ramp vapor sort half answer drill exact turtle city mix next sketch box subject enable select easily moral weekend immune mistake cry case"
ADDRESS7="stars1akeypwqgmhxf3vqv7luml4ccraywrjynpmj5qt"

MNEMONIC8="duck wealth arrest dice popular sustain apart sign lava fury pizza spider panic design jacket autumn fork floor stadium match cat spend relax mention"
ADDRESS8="stars1dulxvxz8489j8t4a5e8dffzchqzflu8scxe28s"

MNEMONIC9="address walnut shuffle project stock novel merge wasp iron sight foot confirm shove kind injury fly post witness math ugly device surprise relief engine"
ADDRESS9="stars122e6mr6uxk20sna2n26gup35fuagxkp5yuu70h"

MNEMONIC10="patrol prison used evolve scan click flip swap reject pipe entire clerk better film fortune shrimp real amateur verb citizen drink wolf hat tag"
ADDRESS10="stars12mp3ms6n5pvx7a03uflxsyr686zzmjc6zhwf7k"

NODES="[\"$ADDRESS1\",\"$ADDRESS2\",\"$ADDRESS3\",\"$ADDRESS4\",\"$ADDRESS5\",\"$ADDRESS6\",\"$ADDRESS7\",\"$ADDRESS8\",\"$ADDRESS9\",\"$ADDRESS10\"]"

ADDRESS=$(eval echo \$ADDRESS$ADDRESS_INDEX)
MNEMONIC=$(eval echo \$MNEMONIC$ADDRESS_INDEX)

# only first run
# beaker wasm deploy oracle-data-aggregator --raw '{ "nois_proxy": "stars1atajhwmu769z6kp2c4htj3qxxex29rwdh55e686fm4dqc6hz80dsxeld4v", "nodes":'${NODES}', "threshold": "5" }' --signer-mnemonic "${MNEMONIC1}" --network stargaze
# beaker wasm execute oracle-data-aggregator --raw '{ "initiate_new_round": {} }' --signer-mnemonic "${MNEMONIC1}" --network stargaze
# exit 1

# workaround if Nois Network is down
# ROUND_ID=1345286
# curl https://rpc.elgafar-1.stargaze-apis.com:443/status | grep -E "height|catching"
# beaker wasm execute oracle-data-aggregator --raw '{ "receive": { "callback": { "randomness": "b5d7d24e428c1234b5d7d24e428c1234b5d7d24e428c1234b5d7d24e428c1234", "job_id": "'${ROUND_ID}'" } } }' --signer-mnemonic "${MNEMONIC1}" --network stargaze

# available queries
# beaker wasm query oracle-data-aggregator --raw '{ "query_all_values": {}}' --network stargaze
# beaker wasm query oracle-data-aggregator --raw '{ "get_history_of_rounds": {}}' --network stargaze
# beaker wasm query oracle-data-aggregator --raw '{ "query_last_round_id": {}}' --network stargaze

declare -A submissions

while true
do
    LAST_ROUND_ID=$( beaker wasm query oracle-data-aggregator --raw '{ "query_last_round_id": {}}' --network stargaze | grep -A1 data | tail -1 )
    LAST_ROUND_ID="$(echo -e "${LAST_ROUND_ID}" | tr -d '[:space:]')"
    echo "Last round id: $LAST_ROUND_ID"

    PRICES=$( cat finalPrices.txt )

    IS_SELECTED=$( beaker wasm query oracle-data-aggregator --raw '{ "query_is_selected": { "round_id": "'${LAST_ROUND_ID}'","node": "'${ADDRESS}'" }}' --network stargaze | grep true)
	if [ -z "$IS_SELECTED" ]; then
        echo "$ADDRESS_INDEX: Is not selected"
    else
        echo "$ADDRESS_INDEX: Is selected"
        
         if [ -z "${submissions[$LAST_ROUND_ID]}" ]; then
            echo ": Submitting..."
            beaker wasm execute oracle-data-aggregator --raw '{ "add_oracle_value": { "update": { "round_id": "'${LAST_ROUND_ID}'", "values": '${PRICES}' } } }' --signer-mnemonic "${MNEMONIC}" --network stargaze
            submissions=( [$LAST_ROUND_ID]="true")
        else
            echo "$ADDRESS_INDEX: Has already submitted"
        fi
    fi

    echo "Waiting for next round..."
	sleep 30
done
