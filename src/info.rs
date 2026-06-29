#[macro_export]
macro_rules! info {
    ($x:expr)=>{
        let now = Local::now();
        let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("{}[INF]:{}",formatted_time,$x);
    }
}