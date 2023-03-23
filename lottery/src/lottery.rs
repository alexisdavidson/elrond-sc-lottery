#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Lottery {
    #[init]
    fn init(
        &self,
        ticket_price: BigUint,
        max_plays_per_wallet: u64,
        rew_1: u64,
        rew_2: u64,
        rew_3: u64,
        rew_4: u64,
        rew_5: u64, // 10, 100, 50, 839, 1,
        nft_manager_address: ManagedAddress,
        nft_token_id: EgldOrEsdtTokenIdentifier,
        opt_token_id: OptionalValue<EgldOrEsdtTokenIdentifier>,
    ) {
        require!(ticket_price > 0, "Ticket price cannot be set to zero");
        self.ticket_price().set(&ticket_price);

        let token_id = match opt_token_id {
            OptionalValue::Some(t) => t,
            OptionalValue::None => EgldOrEsdtTokenIdentifier::egld(),
        };
        self.accepted_payment_token_id().set(&token_id);
        self.nft_token_id().set(&nft_token_id);
        self.nft_manager_address().set(&nft_manager_address);

        for i in 0..rew_1 {
            self.rew_vec().push(&1);
        }
        for i in 0..rew_2 {
            self.rew_vec().push(&2);
        }
        for i in 0..rew_3 {
            self.rew_vec().push(&3);
        }
        for i in 0..rew_4 {
            self.rew_vec().push(&4);
        }
        for i in 0..rew_5 {
            self.rew_vec().push(&5);
        }

        let totalSupply = (rew_1 + rew_2 + rew_3 + rew_4 + rew_5) as u64;
        self.total_supply().set(&totalSupply);
        self.remaining_supply().set(&totalSupply);

        self.max_plays_per_wallet().set(&max_plays_per_wallet);
    }

    // endpoints
    
    /// Buy lottery ticket
    #[payable("*")]
    #[endpoint]
    fn buy_ticket(&self) {
        let caller = self.blockchain().get_caller();
        let user_amount_played = self.user_amount_played(&caller).get();
        let max_plays_per_wallet = self.max_plays_per_wallet().get();

        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        require!(payment_token == self.accepted_payment_token_id().get() 
            || payment_token == self.nft_token_id().get(), "Invalid payment token");
        
        if (payment_token == self.accepted_payment_token_id().get()) {
            require!(payment_amount == self.ticket_price().get(), "The payment must match the ticket price");
        } else {
            require!(payment_amount == 1, "Only a single NFT should be burnt to play");
            // todo: Burn nft
        }
        require!(max_plays_per_wallet == 0 || user_amount_played < max_plays_per_wallet, "Wallet played maximum amount");

        let is_a_sc = self.blockchain().is_smart_contract(&self.blockchain().get_caller());
        require!(!is_a_sc, "Cannot call this function from a smart contract");

        let current_block_timestamp = self.blockchain().get_block_timestamp();
        let reward_index = current_block_timestamp % (self.rew_vec().len()) as u64 + 1_u64;
        let reward = self.rew_vec().get(reward_index as usize);

        self.user_amount_played(&caller).set(&(user_amount_played + 1));

        self.rew_vec().swap_remove(reward_index as usize);
        self.participants().push(&caller);
        self.participants_reward().push(&reward);
        self.remaining_supply().set(self.remaining_supply().get() - 1);

        self.reward_event(&caller, &reward);
    }
    
    /// Set ticket price
    #[only_owner]
    #[endpoint]
    fn set_price(&self, ticket_price: BigUint ) {
        require!(ticket_price > 0, "Ticket price cannot be set to zero");
        self.ticket_price().set(&ticket_price);
    }
    
    // Claim balance
    #[only_owner]
    #[endpoint]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();
        let egld_balance = self
            .blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0);

        self.send().direct_egld(&caller, &egld_balance);
    }

    // views
    // ...

    // storage

    #[view(getAcceptedPaymentToken)]
    #[storage_mapper("acceptedPaymentTokenId")]
    fn accepted_payment_token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getNftToken)]
    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getNftManagerAddress)]
    #[storage_mapper("nftManagerAddress")]
    fn nft_manager_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTicketPrice)]
    #[storage_mapper("ticketPrice")]
    fn ticket_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalSupply)]
    #[storage_mapper("totalSupply")]
    fn total_supply(&self) -> SingleValueMapper<u64>;

    #[view(getMaxPlaysPerWallet)]
    #[storage_mapper("maxPlaysPerWallet")]
    fn max_plays_per_wallet(&self) -> SingleValueMapper<u64>;

    #[view(getRemainingSupply)]
    #[storage_mapper("remainingSupply")]
    fn remaining_supply(&self) -> SingleValueMapper<u64>;

    #[view(getRemainingRewards)]
    #[storage_mapper("remainingRewards")]
    fn rew_vec(&self) -> VecMapper<u64>;

    #[view(getParticipants)]
    #[storage_mapper("participants")]
    fn participants(&self) -> VecMapper<ManagedAddress>;

    #[view(getParticipantsReward)]
    #[storage_mapper("participantsReward")]
    fn participants_reward(&self) -> VecMapper<u64>;

    #[view(getAserAmountPlayed)]
    #[storage_mapper("userAmountPlayed")]
    fn user_amount_played(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    // events

    #[event("rewardEvent")]
    fn reward_event(&self, #[indexed] user: &ManagedAddress, rew: &u64);
}
