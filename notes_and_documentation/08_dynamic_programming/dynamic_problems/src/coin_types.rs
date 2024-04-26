pub struct CoinTypes {
    pub euro_coins: [u32;8],
    pub odd_coins: [u32;3],
}

impl Default for CoinTypes {
    fn default() -> Self {
        Self {
            euro_coins: [ 200, 100, 50, 20, 10, 5, 2, 1 ],
            odd_coins: [ 5, 4, 1 ]
        }
    }
}

impl CoinTypes {
    pub fn new() -> Self {
        CoinTypes::default()
    }

    // simplest type, easy to get the minimum amount of coins from these values
    pub fn get_euro_coins_target(&self, euro_coins_amounts: &mut [u32;8], target_num: u32) {
        let mut target_sum = 0;
        let mut idx = 0;

        // coins already in order from greatest to least
        for coin in self.euro_coins {
            let divide_amount: u32 = (target_num - target_sum) / coin;
            target_sum += coin * divide_amount;
            euro_coins_amounts[idx] += divide_amount;
            idx += 1;
        }
    }

    pub fn get_odd_coins_target(&self, odd_coins_amounts: &mut [u32;3], target_num: u32) {
        let mut target_sum = 0;
        let mut idx = 0;

        // coins already in order from greatest to least
        for coin in self.odd_coins {
            let divide_amount: u32 = (target_num - target_sum) / coin;
            target_sum += coin * divide_amount;
            odd_coins_amounts[idx] += divide_amount;
            idx += 1;
        }
    }
}