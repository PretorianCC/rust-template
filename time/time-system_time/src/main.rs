// Системное время.
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn main() {

    // Константа определяется как «1970-01-01 00:00:00 UTC» для всех систем по отношению к системным часам.
    let now = SystemTime::UNIX_EPOCH;
    println!("SystemTime::UNIX_EPOCH {:?}", now); // SystemTime::UNIX_EPOCH SystemTime { intervals: 116444736000000000 }

    // Текущее время системных часов.
    let now = SystemTime::now();
    println!("SystemTime::now {:?}", now); // SystemTime::now SystemTime { intervals: 133167677746926970 }

    // Прошедшее время системных часов.
    let now = SystemTime::now();
    sleep(Duration::from_secs(1));
    println!("elapsed {:?}", now.elapsed()); // elapsed Ok(1.0148211s)

    // Сумма системных часов и интервала времени.
    let now = SystemTime::now();
    println!("checked_add {:?}", now.checked_add(Duration::from_secs(1))); // checked_add Some(SystemTime { intervals: 133167823052683652 })

    // Разность системных часов и интервала времени.
    let now = SystemTime::now();
    println!("checked_sub {:?}", now.checked_add(Duration::from_secs(1))); // checked_sub Some(SystemTime { intervals: 133167823851737969 })

    // Разность системных часов и интервала времени.
    let now = SystemTime::now();
    println!("checked_sub {:?}", now.checked_add(Duration::from_secs(1))); // checked_sub Some(SystemTime { intervals: 133167823851737969 })

    // Интервал разницы системных часов.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!("duration_since {:?}", now2.duration_since(now1)); // duration_since Ok(100ns)

    // Сумма системных часов и интервала времени. Паника при переполнении.
    let now = SystemTime::now();
    let duration = Duration::from_secs(3);
    println!("+ {:?}", now + duration); // + SystemTime { intervals: 133167825844267215 }

    // Добавить к системным часам интервал времени. Паника при переполнении.
    let mut now = SystemTime::now();
    let duration = Duration::from_secs(3);
    now += duration;
    println!("+= {:?}", now); // += SystemTime { intervals: 133167826531991354 }
    
    // Клонирует системное время.
    let now1 = SystemTime::now();
    let now2 = now1.clone();
    println!("clone {:?}", now2); // clone SystemTime { intervals: 133167993061069849 }

    // Клонирует системное время.
    let now1 = SystemTime::now();
    let mut now2 = SystemTime::now();
    now2.clone_from(&now1);
    println!("clone_from {:?}", now2); // clone_from SystemTime { intervals: 133167993780353729 }

    // Передает системное время в заданный Hasher.
    let now = SystemTime::now();
    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);
    println!("hash {:?}", hasher.finish()); // hash 15662580301247108187

    // Сравнить системное время.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!("cmp {:?}", now1.cmp(&now2)); // cmp Less

    // Максимальное системное время.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!("max {:?}", now1.max(now2)); // max SystemTime { intervals: 133167996289741926 }

    // Минимальное системное время.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!("min {:?}", now1.min(now2)); // min SystemTime { intervals: 133167996766635488 }

    // self если он входит в диапазон min-max, иначе возвращает минимальное или максимальное. Паника если min > max.
    let now_min = SystemTime::now();
    let now = SystemTime::now();
    let now_max = SystemTime::now();
    println!("clamp {:?}", now.clamp(now_min, now_max)); // clamp SystemTime { intervals: 133167997243704001 }

    // Проверка на равенство.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!("== {:?}", now1 == now2); // == false

    // Проверка на не равенство.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!("!= {:?}", now1 != now2); // != true

    // Меньше.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!("< {:?}", now1 < now2); // < true

    // Меньше или равно.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!("<= {:?}", now1 <= now2); // <= true

    // Больше.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!("> {:?}", now1 > now2); // > false

    // Больше или равно.
    let now1 = SystemTime::now();
    let now2 = SystemTime::now();
    println!(">= {:?}", now1 >= now2); // >= false

    // Отнять от системного времени период времени.
    let now = SystemTime::now();
    let duration = Duration::new(1, 0);
    println!("- {:?}", now - duration); // - SystemTime { intervals: 133168002177466742 }

    // Уменьшить момент на период времени.
    let mut now = SystemTime::now();
    let duration = Duration::new(1, 0);
    now -= duration;
    println!("-= {:?}", now); // -= SystemTime { intervals: 133168002177466847 }

}
