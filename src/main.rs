use colored::Colorize;
fn main() {
    let colors = ["red", "green", "blue", "yellow", "purple"];
    let path_to_ascii: String = String::from("/etc/mifetch/ascii.txt");
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    let os = sysinfo::System::name().unwrap_or("Unknown System".to_string());
    let ram = round::round(sys.total_memory() as f64/1073741824.0,2 );
    let swap = round::round(sys.total_swap() as f64/1073741824.0, 2);
    let user_ram = round::round(sys.used_memory() as f64/1073741824.0,2);
    let host_name = sysinfo::System::host_name().unwrap_or("Unknown Host".to_string());
    let cpu = sys.cpus().first().unwrap();
    let ascii = std::fs::read_to_string(path_to_ascii).unwrap_or_else(|e| {
        eprintln!("[ERROR]: {}", e);
        "[ERROR]: Can't read ascii.txt. Check /etc/mifetch/ascii.txt file".to_string()
    });
    for line in ascii.lines() {
        if line.starts_with("$"){
            let c = line.chars().nth(1).unwrap().to_digit(10);
            let color_name = colors[c.unwrap() as usize -1];
            let cropped: String = line.chars().skip(2).collect();
            println!("{}", cropped.color(color_name));
        }else {
                println!("{}", line)
            }
    }
    println!("{}: {}", "CPU".bold().red(), cpu.brand());
    println!("{}: {}","OS".bold().green(), os);
    println!("{}: {}GB","RAM".bold().blue(), ram, );
    println!("{}: {}GB","USED RAM".bold().cyan(), user_ram);
    println!("{}: {}GB","SWAP".bold().purple(), swap);
    println!("{}: {}", "HOST".bold().white(), host_name);


}