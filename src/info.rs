#[macro_export]
macro_rules! info {
    ($x:expr)=>{
        use colored::Colorize;
        let now = chrono::Local::now();
        let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let info = "[INFO]";
        println!("{} {}:{}",formatted_time,$x);
    }
}