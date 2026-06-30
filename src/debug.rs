#[macro_export]
macro_rules! debug {
    ($x:expr)=>{
        {
            use $crate::colored::Colorize;
            let now = $crate::chrono::Local::now();
            let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let info = "[DBUG]";
            println!("{} {} : {}",info.blue(),formatted_time.blue(),$x);
        }
    }
}