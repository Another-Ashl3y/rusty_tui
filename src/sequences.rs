#[allow(dead_code, non_snake_case)]

// :x to exit vim

pub fn reset() {
    println!("\u{001b}[0m\u{001b}[1A")
}

pub mod Fore {
    fn fix() {
        println!("\u{001b}[2A");
    }
    pub fn black() {
   	    println!("\u{001b}[30m");
        fix();
    }
    pub fn red() {
        println!("\u{001b}[31m");
        fix();
    }
    pub fn green() {
        println!("\u{001b}[32m");
        fix();
    }
    pub fn yellow() {
        println!("\u{001b}[33m");
        fix();
    }
    pub fn blue() {
        println!("\u{001b}[34m");
        fix();
    }
    pub fn magenta() {
        println!("\u{001b}[35m");
        fix();
    }
    pub fn cyan() {
        println!("\u{001b}[36m");
        fix();
    }
    pub fn white() {
        println!("\u{001b}[37m");
        fix();
    }
    pub fn p_black() {
        println!("\u{001b}[90m");
        fix();
    }
    pub fn p_red() {
        println!("\u{001b}[91m");
        fix();
    }
    pub fn p_green() {
        println!("\u{001b}[92m");
        fix();
    }
    pub fn p_yellow() {
        println!("\u{001b}[93m");
        fix();
    }
    pub fn p_blue() {
        println!("\u{001b}[94m");
        fix();
    }
    pub fn p_magenta() {
        println!("\u{001b}[95m");
        fix();
    }
}

pub mod Back {
    fn fix() {
        println!("\u{001b}[2A");
    }
    pub fn black() {
        println!("\u{001b}[40m");
        fix();
    }
    pub fn red() {
        println!("\u{001b}[41m");
        fix();
    }
    pub fn green() {
        println!("\u{001b}[42m");
        fix();
    }
    pub fn yellow() {
        println!("\u{001b}[43m");
        fix();
    }
    pub fn blue() {
        println!("\u{001b}[44m");
        fix();
    }
    pub fn magenta() {
        println!("\u{001b}[45m");
        fix();
    }
    pub fn cyan() {
        println!("\u{001b}[46m");
        fix();
    }
    pub fn white() {
        println!("\u{001b}[47m");
        fix();
    }
}

pub mod Move {

    pub fn up(n: u32) {
    	println!("\u{001b}[{}A", (n+1).to_string());
    }
    pub fn right(n: u32) {
        println!("\u{001b}[{}C", n.to_string());
        up(1);
    }
    pub fn down(n: u32) {
        println!("\u{001b}[{}B", n.to_string());
        up(1);
    }
    pub fn left(n: u32) {
        println!("\u{001b}[{}D", n.to_string());
        up(1);
    }


}
