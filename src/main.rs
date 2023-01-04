use gacha_simulator::{Wish, RATE_4, RATE_5};

fn main() {
    let mut wish = Wish {
        draw_count_5: 0,
        draw_count_4: 0,
        variable_rate_5: RATE_5,
        variable_rate_4: RATE_5 + RATE_4,
        result_count_5: 0,
        result_count_4: 0,
        result_count_3: 0,
    };

    let mut count: u32 = 0;
    while count < 100_000_000 {
        wish.wish();
        count += 1
    }

    let rarity_5: f64 = ((wish.result_count_5 * 100) / count).into();
    let rarity_4: f64 = ((wish.result_count_4 * 100) / count).into();

    println!(
        "You've got *5 x{}. It's rarity is {}%.",
        wish.result_count_5, rarity_5
    );
    println!(
        "You've got *4 x{}. It's rarity is {}%.",
        wish.result_count_4, rarity_4
    )
}
