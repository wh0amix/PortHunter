use std::io::{self, Write, Read};
use std::net::{TcpStream, SocketAddr, ToSocketAddrs, IpAddr};
use std::time::Duration;
use rayon::prelude::*;
use dns_lookup::lookup_addr;
use std::collections::HashMap;
use figlet_rs::FIGfont;

fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("PortHunter");

    if let Some(ref fig) = figure {
        println!("{}", fig);
    }
}

fn scan_port(ip: &str, port: u16, timeout: Duration) -> Option<String> {
    let addr = format!("{}:{}", ip, port);
    let socket_addrs: Vec<SocketAddr> = match addr.to_socket_addrs() {
        Ok(addrs) => addrs.collect(),
        Err(_) => return None,
    };

    for socket_addr in socket_addrs {
        if let Ok(mut stream) = TcpStream::connect_timeout(&socket_addr, timeout) {
            let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));

            let mut buffer = [0; 512];
            if let Ok(n) = stream.read(&mut buffer) {
                if n > 0 {
                    let banner = String::from_utf8_lossy(&buffer[..n]).to_string();
                    return Some(banner);
                }
            }

            return Some(String::from("Port ouvert, pas de bannière détectée"));
        }
    }

    None
}

fn scan_ports_menu() {
    let mut ip = String::new();
    print!("Entrez l'adresse IP à scanner : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim();
    let hostname = match ip.parse::<IpAddr>() {
        Ok(ip_addr) => match lookup_addr(&ip_addr) {
            Ok(name) => name,
            Err(_) => "Non résolu".to_string(),
        },
        Err(_) => "Adresse invalide".to_string(),
    };

    match ip.parse::<IpAddr>() {
        Ok(ip_addr) => {
            match lookup_addr(&ip_addr) {
                Ok(hostname) => println!("🌐 Nom d’hôte résolu : {}", hostname),
                Err(_) => println!("🌐 Nom d’hôte non trouvé."),
            }
        }
        Err(_) => println!("❌ Adresse IP invalide."),
    }        

    let start_port = 1;
    let end_port = 1024;
    let timeout = Duration::from_secs(1);

    println!("🔍 Scan de {} du port {} à {}", ip, start_port, end_port);

    (start_port..=end_port).into_par_iter().for_each(|port| {
        if let Some(banner) = scan_port(ip, port, timeout) {
            println!("✅ Port {} est ouvert", port);
            println!("   🏷️  Bannière : {}", banner.trim());
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
