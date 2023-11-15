all            :  build-4 build-8
clean          :; dapp clean
test           :; dapp test
build-4        :; DAPP_SRC=src/solc_0.4 DAPP_OUT=out/solc_0.4 DAPP_BUILD_OPTIMIZE_RUNS=200 dapp --use solc:0.4.24 build
build-8        :; DAPP_SRC=src/solc_0.8 DAPP_OUT=out/solc_0.8 DAPP_BUILD_OPTIMIZE_RUNS=200 dapp --use solc:0.8.6  build
deploy-wallet  :; dapp create MultiSigWallet ["0x0f14341A7f464320319025540E8Fe48Ad0fe5aec","0x05e0375b33aeF6a5BC6c02f52E02F62aadc586FE","0x74003cbD6c4f34a8380BB6e8b07914c8f381D864"] 2
deploy-darw    :; dapp create TimeLock 1800 ["0xBd1a110ec476b4775c43905000288881367B1a88"] ["0x00E3993566b34e5367d1C602439997BD08c11FF7","0x52386BE2397e8EAc26298F733b390684203fB580","0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6","0xe59261f6D4088BcD69985A3D369Ff14cC54EF1E5","0xd2c7008400F54aA70Af01CF8C747a4473246593E"]

.PHONY: build-4 build-8
