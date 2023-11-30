all            :  build-4 build-8
clean          :; dapp clean
test           :; dapp test
build-4        :; DAPP_SRC=src/solc_0.4 DAPP_OUT=out/solc_0.4 DAPP_BUILD_OPTIMIZE_RUNS=200 dapp --use solc:0.4.24 build
build-8        :; DAPP_SRC=src/solc_0.8 DAPP_OUT=out/solc_0.8 DAPP_BUILD_OPTIMIZE_RUNS=200 dapp --use solc:0.8.17 build
deploy-wallet  :; dapp create MultiSigWallet ["0x0f14341A7f464320319025540E8Fe48Ad0fe5aec","0x05e0375b33aeF6a5BC6c02f52E02F62aadc586FE","0x74003cbD6c4f34a8380BB6e8b07914c8f381D864"] 2
deploy-timelock:; @./bin/deploy-timelock.sh $(safe) $(chain)

.PHONY: build-4 build-8
