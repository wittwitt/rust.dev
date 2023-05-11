const POOL_LIMIT: i32 = 10000;

static POOL_THREAD: i32 = 0;

static mut POOL_WOKER_COUNT: i32 = 0;

static POOL_NAME: &'static str = "abc";

// static mut S1: String = "".to_string();

// static mut POOL_NAME: &'static mut str = String::from_str("abc").as_ref();

fn main() {
    let mut p = Pool { woker_count: 0 };

    println!("POOL_LIMIT： {:?}", POOL_LIMIT);

    println!("POOL_THREAD: {:?}", POOL_THREAD);

    println!("POOL_NAME: {:?}", POOL_NAME);

    // println!("POOL_NAME: {:?}", POOL_NAME);

    unsafe {
        loop {
            POOL_WOKER_COUNT = POOL_WOKER_COUNT + 1;
            if POOL_WOKER_COUNT > POOL_LIMIT {
                break;
            }
        }
    }
    unsafe {
        println!("POOL_WOKER_COUNT: {:?}", POOL_WOKER_COUNT);
    }

    loop {
        // p.woker_count = p.woker_count + 1;

        let mut new_worker = Worker {
            id: 0,
            name: "zhangsan",
        };
        add_worker(&mut p, &mut new_worker);

        println!("new worker: {:?}, {:?}", new_worker.name, new_worker.id);

        if p.woker_count > POOL_LIMIT {
            break;
        }
    }

    println!("POOL_WOKER_COUNT: {:?}", p.woker_count);
}

struct Pool {
    woker_count: i32,
}

struct Worker {
    id: i32,
    name: &'static str,
}

// fn add_worker<'a, 'b>(pool: &'a mut Pool, woker: &'b Worker) -> &'a mut Pool {
//     println!("new worker: {:?}", woker.name);

//     pool.woker_count = pool.woker_count + 1;
//     pool
// }

fn add_worker(pool: &mut Pool, woker: &mut Worker) {
    pool.woker_count = pool.woker_count + 1;

    woker.id = pool.woker_count;
}

// struct Cat<'a> {
//     name: &'a str,
// }

struct Cat<'a> {
    name: &'a str,
}

#[test]
fn f1() {
    let c1 = &Cat { name: "abc" };
    println!("{}", c1.name);
}
