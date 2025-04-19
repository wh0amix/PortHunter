use std::io::{self, Write};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;
use rayon::prelude::*;
use std::collections::HashMap;
use figlet_rs::FIGfont;

fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("PortHunter");

    if let Some(ref fig) = figure {
        println!("{}", fig);
    }
}

fn scan_port(ip: &str, port: u16, timeout: Duration) -> bool {
    let addr = format!("{}:{}", ip, port);
    let socket_addrs: Vec<SocketAddr> = match addr.to_socket_addrs() {
        Ok(addrs) => addrs.collect(),
        Err(_) => return false,
    };

    for socket_addr in socket_addrs {
        if TcpStream::connect_timeout(&socket_addr, timeout).is_ok() {
            return true;
        }
    }

    false
}

fn scan_ports_menu() {
    let mut ip = String::new();
    print!("Entrez l'adresse IP à scanner : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim();

    let start_port = 1;
    let end_port = 1024;
    let timeout = Duration::from_secs(1);

    println!("🔍 Scan de {} du port {} à {}", ip, start_port, end_port);

    (start_port..=end_port).into_par_iter().for_each(|port| {
        if scan_port(ip, port, timeout) {
            println!("✅ Port {} est ouvert", port);
        }
    });

    println!("✅ Scan terminé !");
}

fn show_port_guide() {
    let mut guide = HashMap::new();
    guide.insert(20, "FTP (Data)");
    guide.insert(21, "FTP (Control)");
    guide.insert(22, "SSH - Secure Shell");
    guide.insert(23, "Telnet - Remote Login");
    guide.insert(25, "SMTP - Email Sending");
    guide.insert(53, "DNS - Domain Name System");
    guide.insert(80, "HTTP - Web Traffic");
    guide.insert(110, "POP3 - Email");
    guide.insert(143, "IMAP - Email");
    guide.insert(443, "HTTPS - Secure Web");
    guide.insert(3306, "MySQL - Database");
    guide.insert(3389, "RDP - Remote Desktop");

    println!("\n📖 Guide des ports courants :");
    for (port, description) in guide.iter() {
        println!("🔹 Port {:<5} → {}", port, description);
    }
    println!();
}

fn main() {
    print_banner();

    loop {
        println!("==============================");
        println!("🔧 Menu Principal");
        println!("1️⃣  Scanner les ports");
        println!("2️⃣  Afficher le guide des ports");
        println!("0️⃣  Quitter");
        println!("==============================");
        print!("👉 Choix : ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => scan_ports_menu(),
            "2" => show_port_guide(),
            "0" => {
                println!("👋 Au revoir !");
                break;
            }
            _ => println!("❌ Option invalide, réessaie."),
        }
    }
}
