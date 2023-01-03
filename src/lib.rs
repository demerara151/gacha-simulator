use rand::Rng;

// 固定レート
pub const RATE_4: f64 = 0.051;
pub const RATE_5: f64 = 0.006;

// 固定新レート
pub const RATE_4_UP: f64 = 0.511;
pub const RATE_5_UP: f64 = 0.324;

pub struct Wish {
    // 引いた回数
    pub draw_count_5: i32,
    pub draw_count_4: i32,

    // 可変レート
    pub variable_rate_5: f64,
    pub variable_rate_4: f64,

    // 結果
    pub result_count_5: i32,
    pub result_count_4: i32,
    pub result_count_3: i32,
}

impl Wish {
    /// 引いた回数によって可変レートの値を決める                                                                                                              
    pub fn wish(&mut self) {
        self.get_rarity();

        if self.draw_count_5 < 75 {
            // 74回目まで固定レート
            self.variable_rate_5 = RATE_5
        } else if self.draw_count_5 < 89 {
            // 75回引くと確率上昇
            self.variable_rate_5 = RATE_5_UP
        } else {
            // 89回目で *5 確定
            self.variable_rate_5 = 1.0
        }

        if self.draw_count_4 < 8 {
            // 7回目まで固定レート
            self.variable_rate_4 = self.variable_rate_5 + RATE_4
        } else if self.draw_count_4 < 9 {
            // 8回引くと確率上昇
            self.variable_rate_4 = self.variable_rate_5 + RATE_4_UP
        } else {
            // 9回目で *4 確定
            self.variable_rate_4 = 1.0
        }
    }

    /// 乱数に応じて結果を算出し、引いた回数をリセット                                                                                                        
    fn get_rarity(&mut self) {
        let random_number: f64 = rand::thread_rng().gen();

        // 可変レートの値より乱数が低ければ当たり
        if random_number <= self.variable_rate_5 {
            self.draw_count_4 += 1;
            self.draw_count_5 = 0;
            self.result_count_5 += 1;
        } else if random_number <= self.variable_rate_4 {
            self.draw_count_4 = 0;
            self.draw_count_5 += 1;
            self.result_count_4 += 1;
        } else {
            self.draw_count_4 += 1;
            self.draw_count_5 += 1;
            self.result_count_3 += 1;
        }
    }
}
