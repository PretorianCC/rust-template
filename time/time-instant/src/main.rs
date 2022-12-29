use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};
use std::thread::sleep;

// Текущий момент.
fn main() {
    
    // Текущий момент времени.
    let now = Instant::now();
    println!("Instant::now {:?}", now); // Instant::now Instant { t: 890560.5731299s }

    // Прошедшее время от инициализации текущего момента.
    let now = Instant::now();
    println!("elapsed {:?}", now.elapsed()); // elapsed 200ns

    // Сумма текущего момента и интервала времени.
    let now = Instant::now();
    let duration = Duration::from_secs(3);
    println!("checked_add {:?} - {:?}", now, now.checked_add(duration)); // checked_add Instant { t: 891121.812458s } - Some(Instant { t: 891124.812458s })

    // Разность текущего момента и интервала времени.
    let now = Instant::now();
    let duration = Duration::from_secs(3);
    println!("checked_sub {:?} - {:?}", now,now.checked_sub(duration)); // checked_sub Instant { t: 891121.8124664s } - Some(Instant { t: 891118.8124664s })

    // Результат разницы текущих моментов или None если она отрицательная.
    let now = Instant::now();
    sleep(Duration::new(1, 0));
    let new_now = Instant::now();
    println!("checked_duration_since {:?}", new_now.checked_duration_since(now)); // checked_duration_since Some(1.0005646s)

    // Разница текущих моментов или 0 если она отрицательная.
    let now = Instant::now();
    sleep(Duration::new(1, 0));
    let new_now = Instant::now();
    println!("duration_since {:?}", new_now.duration_since(now)); // duration_since 1.0107662s

    // Разница текущих моментов как интервал времени или 0 если она отрицательная.
    let now = Instant::now();
    sleep(Duration::new(1, 0));
    let new_now = Instant::now();
    println!("saturating_duration_since {:?}", new_now.saturating_duration_since(now)); // saturating_duration_since 1.0033914s

    // Сумма текущего момента и интервала времени. Паника при переполнении.
    let now = Instant::now();
    let duration = Duration::from_secs(3);
    println!("+ {:?}", now + duration); // + Instant { t: 971572.4393983s }
    
    // Добавить к текущему моменту интервал времени. Паника при переполнении.
    let mut now = Instant::now();
    let duration = Duration::from_secs(3);
    now += duration;
    println!("+= {:?}", now); // += Instant { t: 971655.7829336s }

    // Клонирует текущий момент.
    let now1 = Instant::now();
    let now2 = now1.clone();
    println!("clone {:?}", now2); // clone Instant { t: 971834.2144548s }

    // Клонирует текущий момент.
    let now1 = Instant::now();
    sleep(Duration::new(1, 0));
    let mut now2 = Instant::now();
    now2.clone_from(&now1);
    println!("clone_from {:?}", now2); // clone_from Instant { t: 972055.7562721s }

    // Передает текущий момент в заданный Hasher.
    let now = Instant::now();
    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);
    println!("hash {:?}", hasher.finish()); // hash 11339876402824030755

    // Сравнить моменты.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("cmp {:?}", now1.cmp(&now2)); // cmp Less

    // Максимальный момент.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("max {:?}", now1.max(now2)); // max Instant { t: 994840.8428835s }

    // Минимальный момент.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("min {:?}", now1.min(now2)); // min Instant { t: 994840.8430649s }

    // self если он входит в диапазон min-max, иначе возвращает минимальное или максимальное. Паника если min > max.
    let now_min = Instant::now();
    let now = Instant::now();
    let now_max = Instant::now();
    println!("clamp {:?}", now.clamp(now_min, now_max)); // clamp Instant { t: 995031.7006968s }

    // Проверка на равенство.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("== {:?}", now1 == now2); // == false

    // Проверка на не равенство.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("!= {:?}", now1 != now2); // != true

    // Сравнение.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("partial_cmp {:?}", now1.partial_cmp(&now2)); // partial_cmp Some(Less)

    // Меньше.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("< {:?}", now1 < now2); // < true

    // Меньше или равно.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("<= {:?}", now1 <= now2); // <= true

    // Больше.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("> {:?}", now1 > now2); // > false

    // Больше или равно.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!(">= {:?}", now1 >= now2); // >= false

    // Отнять от момента период времени.
    let now = Instant::now();
    let duration = Duration::new(1, 0);
    println!("- {:?}", now - duration); // - Instant { t: 995708.3398491s }

    // Период времени между моментами.
    let now1 = Instant::now();
    let now2 = Instant::now();
    println!("- {:?}", now2 - now1); // - 100ns

    // Уменьшить момент на период времени.
    let mut now = Instant::now();
    let duration = Duration::new(1, 0);
    now -= duration;
    println!("-= {:?}", now); // -= Instant { t: 995850.4417094s }

}
