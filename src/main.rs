fn main() {
    let PATH_TO_ASCII: String = String::from("ascii.txt");
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    // получаю данные
    let os = sysinfo::System::name().unwrap_or("Unknown System".to_string());
    let ram = round::round(sys.total_memory() as f64/1073741824.0,2 );
    let swap = round::round(sys.total_swap() as f64/1073741824.0, 2);
    let user_ram = round::round(sys.used_memory() as f64/1073741824.0,2);
    let host_name = sysinfo::System::host_name().unwrap_or("Unknown Host".to_string());
    let cpu = sys.cpus().first().unwrap();
    // читаю ascii.txt
    let mut ascii: Result<String, std::io::Error> = std::fs::read_to_string(&PATH_TO_ASCII);
    ascii = Ok(ascii.unwrap_or_else(|e| {
        String::from("[ERROR]: can't find ascii.txt")
    }));
    // вывожу
    println!("{}", ascii.unwrap());
    println!("CPU: {}", cpu.brand());
    println!("OS: {}", os);
    println!("RAM: {}GB", ram, );
    println!("USED RAM: {}GB", user_ram);
    println!("SWAP: {}GB", swap);
    println!("HOST: {}", host_name);


}
