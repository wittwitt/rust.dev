use rayon::prelude::*;

use std::panic::{self, AssertUnwindSafe};

fn main() {
    println!("Hello, world!");
    ass();
    println!("end");
}

fn sss() {
    panic!("this is sss")
}

fn ass() {
    let dd: i32 = (0..4)
        .into_par_iter()
        .map(|i| {
            let j = match panic::catch_unwind(AssertUnwindSafe(|| {
                if i > 2 {
                    // panic!("cccc: {}", i);
                    sss();
                    0
                } else {
                    1
                }
            })) {
                Ok(_i) => {
                    println!("===1",);
                    1
                }
                Err(e) => {
                    println!("ppp: {:?}", e);

                    2
                }
            };

            match panic::catch_unwind(AssertUnwindSafe(|| {
                panic!("===================: {}", i);
            })) {
                Ok(_i) => j,
                Err(_e) => 1,
            }
        })
        .sum();

    println!("{}", dd);
}

mod test {

    use std::iter::FromIterator;

    use anyhow::Result;
    use rayon::{
        iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator},
        slice::ParallelSliceMut,
    };

    #[test]
    fn t1() {
        let end: u64 = 10;

        (1..end).step_by(4).for_each(|item| {
            println!("step_by: {:?}", item);
        });

        let v1 = Vec::from_iter((1..end).step_by(4));
        let v2 = vec![100, 200, 300];

        v1.into_par_iter()
            .zip(v2)
            .try_for_each(|(index, item)| -> Result<()> {
                println!("{},,,{}", index, item);
                Ok(())
            })
            .unwrap();
    }
}
