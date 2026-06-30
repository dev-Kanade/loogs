#[macro_export]
macro_rules! wern {
    ($x:expr)=>{
        {
            use $crate::colored::Colorize;
            let now = $crate::chrono::Local::now();
            let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let info = "[WERN]";
            println!("{} {} : {}",info.yellow(),formatted_time.yellow(),$x);
        }
    }
}