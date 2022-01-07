#[macro_export]
macro_rules! timeit {
    ($module:ident, $func:ident, $method:ident) => {
        pub fn $func() -> String {
            use std::time;
            let now = time::Instant::now();
            let ans = $method();
            let exec_time = now.elapsed().as_secs_f32();
            println!(
                "{}: the solution is {}. The execution took {} seconds",
                stringify!($module),
                ans,
                exec_time
            );
            ans
        }
    };
}

pub(crate) use timeit;
