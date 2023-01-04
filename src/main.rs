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

    for _ in 1..100_000_000 {
        wish.wish();
    }

    println!("You've got *5 x{}", wish.result_count_5);
    println!("You've got *4 x{}", wish.result_count_4)
}
