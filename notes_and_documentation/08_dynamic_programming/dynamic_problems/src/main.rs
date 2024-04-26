use coin_types::CoinTypes;

pub mod coin_types;

fn main() {
    let coins: CoinTypes = CoinTypes::new();
    let mut euro_coins_amounts: [u32;8] = [0;8];
    let mut odd_coins_amounts: [u32;3] = [0;3];

    coins.get_euro_coins_target(&mut euro_coins_amounts, 734);
    println!("{:?} = {:?}", euro_coins_amounts, get_sum_from_euro_coins_array(&euro_coins_amounts));

    coins.get_odd_coins_target(&mut odd_coins_amounts, 13);
    println!("{:?} = {:?}", odd_coins_amounts, get_sum_from_odd_coins_array(&odd_coins_amounts));
}

fn get_sum_from_euro_coins_array(euro_coins_amounts: &[u32;8]) -> u32 {
    let mut sum: u32 = 0;

    for count in euro_coins_amounts {
        sum += count;
    }

    return sum;
}

fn get_sum_from_odd_coins_array(odd_coins_amounts: &[u32;3]) -> u32 {
    let mut sum: u32 = 0;

    for count in odd_coins_amounts {
        sum += count;
    }

    return sum;
}

