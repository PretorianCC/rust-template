use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

fn main() {

    // Константа максимального представления продолжительности времени.
    let duration = Duration::MAX;
    println!("MAX {:?}", duration); // MAX 18446744073709551615.999999999s
    
    // Константа минимального представления продолжительности времени.
    let duration = Duration::ZERO;
    println!("ZERO {:?}", duration); // ZERO 0ns

    // Количество целых микросекунд.
    let duration = Duration::new(5, 730023852);
    println!("as_micros {:?}", duration.as_micros()); // as_micros 5730023

    // Количество целых миллисекунд.
    let duration = Duration::new(5, 730023852);
    println!("as_millis {:?}", duration.as_millis()); // as_millis 5730

    // Количество наносекунд.
    let duration = Duration::new(5, 730023852);
    println!("as_nanos {:?}", duration.as_nanos()); // as_nanos 5730023852

    // Количество секунд.
    let duration = Duration::new(5, 730023852);
    println!("as_secs {:?}", duration.as_secs()); // as_secs 5

    // Количество секунд.
    let duration = Duration::new(5, 730023852);
    println!("as_secs_f32 {:?}", duration.as_secs_f32()); // as_secs_f32 5.730024

    // Количество секунд.
    let duration = Duration::new(5, 730023852);
    println!("as_secs_f64 {:?}", duration.as_secs_f64()); // as_secs_f64 5.730023852

    // Сложение.
    let duration = Duration::new(0, 0).checked_add(Duration::new(0, 1));
    println!("checked_add {:?}", duration); // checked_add Some(1ns)

    // Деление.
    let duration = Duration::new(2, 0).checked_div(2);
    println!("checked_div {:?}", duration); // checked_div Some(1s)

    // Умножение.
    let duration = Duration::new(0, 500_000_001).checked_mul(2);
    println!("checked_mul {:?}", duration); // checked_mul Some(1.000000002s)

    // Разность.
    let duration = Duration::new(0, 1).checked_sub(Duration::new(0, 0));
    println!("checked_sub {:?}", duration); // checked_sub Some(1ns)

    // Деление.
    let duration = Duration::new(2, 700_000_000);
    println!("div_f32 {:?}", duration.div_f32(3.14)); // div_f32 859.87258ms

    // Деление.
    let duration = Duration::new(2, 700_000_000);
    println!("div_f64 {:?}", duration.div_f64(3.14)); // div_f64 859.872611ms

    // Продолжительность времени из указанного числа микросекунд.
    let duration = Duration::from_micros(1_000_002);
    println!("from_micros {:?}", duration); // from_micros 1.000002s

    // Продолжительность времени из указанного числа миллисекунд.
    let duration = Duration::from_millis(2569);
    println!("from_millis {:?}", duration); // from_millis 2.569s

    // Продолжительность времени из указанного числа наносекунд.
    let duration = Duration::from_nanos(1_000_000_123);
    println!("from_nanos {:?}", duration); // from_nanos 1.000000123s
   
    // Продолжительность времени из указанного числа секунд.
    let duration = Duration::from_secs(5);
    println!("from_secs {:?}", duration); // from_secs 5s

    // Продолжительность времени из указанного числа секунд.
    let duration = Duration::from_secs_f32(3e10);
    println!("from_secs_f32 {:?}", duration); // from_secs_f32 30000001024s

    // Продолжительность времени из указанного числа секунд.
    let duration = Duration::from_secs_f64(3e10);
    println!("from_secs_f64 {:?}", duration); // from_secs_f64 30000000000s

    // true, если продолжительность 0.
    let duration = Duration::from_secs(0);
    println!("is_zero {:?}", duration.is_zero()); // is_zero true

    // Умножение.
    let duration = Duration::new(2, 700_000_000);
    println!("mul_f32 {:?}", duration.mul_f32(3.14)); // mul_f32 8.478000641s

    // Умножение.
    let duration = Duration::new(2, 700_000_000);
    println!("mul_f64 {:?}", duration.mul_f64(3.14)); // mul_f64 8.478s

    // Новая продолжительность времени.
    let duration = Duration::new(2, 700_000_000);
    println!("Duration::new {:?}", duration); // Duration::new 2.7s

    // Сложение, при переполнении Duration::MAX.
    let duration = Duration::new(2, 700_000_000);
    println!("saturating_add {:?}", duration.saturating_add(Duration::new(0, 1))); // saturating_add 2.700000001s

    // Умножение, при переполнении Duration::MAX.
    let duration = Duration::new(2, 700_000_000);
    println!("saturating_mul {:?}", duration.saturating_mul(2)); // saturating_mul 5.4s

    // Разность, при переполнении Duration::ZERO.
    let duration = Duration::new(2, 700_000_000);
    println!("saturating_sub {:?}", duration.saturating_sub(Duration::new(0, 1))); // saturating_sub 2.699999999s

    // Дробная часть длительности в целых микросекундах.
    let duration = Duration::from_micros(1_234_567);
    println!("subsec_micros {:?}", duration.subsec_micros()); // subsec_micros 234567

    // Дробная часть длительности в целых миллисекундах.
    let duration = Duration::from_millis(5432);
    println!("subsec_millis {:?}", duration.subsec_millis()); // subsec_millis 432

    // Дробная часть длительности в целых наносекундах.
    let duration = Duration::from_millis(5432);
    println!("subsec_nanos {:?}", duration.subsec_nanos()); // subsec_nanos 432000000

    // Сложение.
    let duration = Duration::new(2, 0) + Duration::new(2, 0);
    println!("+ {:?}", duration); // + 4s

    // Сложение с присвоением.
    let mut duration = Duration::new(2, 0);
    duration += duration;
    println!("+= {:?}", duration); // += 4s

    // Клонирование.
    let duration = Duration::new(2, 0);
    let result = duration.clone();
    println!("clone {:?}", result); // clone 2s

    // Клонирование в текущую длительность.
    let mut duration = Duration::new(0, 0);
    let result = Duration::new(2, 0);
    duration.clone_from(&result);
    println!("clone_from {:?}", result); // clone_from 2s

    // Длительность по умолчанию.
    let duration: Duration = Default::default();
    println!("Default::default {:?}", duration); // Default::default 0ns

    // Деление с присваеванием.
    let mut duration = Duration::new(2, 0);
    duration /= 2;
    println!("/= {:?}", duration); // /= 1s

    // Передает это значение в заданный Hasher.
    let duration = Duration::new(2, 0);
    let mut hasher = DefaultHasher::new();
    duration.hash(&mut hasher);
    println!("hash {:?}", hasher.finish()); // hash 13159505960806080403

    // Умножение.
    let duration = Duration::new(2, 0);
    println!("* {:?}", duration * 2); // * 4s

    // Произведение с присвоением.
    let mut duration = Duration::new(2, 0);
    duration *= 2;
    println!("*= {:?}", duration); // *= 4s

    // Сравнение.
    let duration1 = Duration::new(2, 0);
    let duration2 = Duration::new(2, 0);
    println!("cmp {:?}", duration1.cmp(&duration2)); // cmp Equal

    // Максимальное.
    let duration1 = Duration::new(2, 0);
    let duration2 = Duration::new(3, 0);
    println!("max {:?}", duration1.max(duration2)); // max 3s

    // Минимальное.
    let duration1 = Duration::new(2, 0);
    let duration2 = Duration::new(3, 0);
    println!("min {:?}", duration1.min(duration2)); // min 2s

    // self если он входит в диапазон min-max, иначе возвращает минимальное или максимальное. Паника если min > max.
    let duration_min = Duration::new(1, 0);
    let duration_max = Duration::new(3, 0);
    let duration = Duration::new(0, 0);
    println!("clamp {:?}", duration.clamp(duration_min, duration_max)); // clamp 1s

    // Равенство.
    let duration = Duration::new(0, 0);
    println!("== {:?}", duration == duration); // == true

    // Не равенство.
    let duration = Duration::new(0, 0);
    println!("!= {:?}", duration != duration); // != false

    // Меньше.
    let duration = Duration::new(0, 0);
    println!("< {:?}", duration < duration); // < false

    // Меньше или равно.
    let duration = Duration::new(0, 0);
    println!("<= {:?}", duration <= duration); // <= true

    // Меньше.
    let duration = Duration::new(0, 0);
    println!("> {:?}", duration > duration); // > false

    // Меньше или равно.
    let duration = Duration::new(0, 0);
    println!(">= {:?}", duration >= duration); // >= true

    // Разность.
    let duration = Duration::new(2, 0);
    println!("- {:?}", duration - duration); // - 0ns

    // Разность с присваением.
    let mut duration = Duration::new(2, 0);
    duration -= duration;
    println!("-= {:?}", duration); // -= 0ns

    // Сложение колекции продолжительности времени.
    let durations = [Duration::new(2, 0), Duration::new(3, 0)];
    let result: Duration = durations.iter().sum();
    println!("sum {:?}",  result); // sum 5s

}
