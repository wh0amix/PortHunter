use std::env;

mod cli;

use std::io::Read;
use std::net::{TcpStream, SocketAddr, ToSocketAddrs, IpAddr};
use std::time::Duration;
use dns_lookup::lookup_addr;
use std::collections::HashMap;
use figlet_rs::FIGfont;
use eframe::egui;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ScanResult {
    port: u16,
    service: String,
    banner: String,
    timestamp: String,
    is_open: bool,
}

#[derive(Clone, Debug)]
struct ScanProgress {
    current_port: u16,
    total_ports: u16,
    scanned: u16,
    open_ports: u16,
    is_scanning: bool,
    hostname: String,
}

#[derive(Clone, Debug)]
enum ScanMessage {
    Progress(ScanProgress),
    Result(ScanResult),
    Complete,
    Error(String),
}

struct PortHunterApp {
    // Configuration
    target_ip: String,
    start_port: String,
    end_port: String,
    timeout_ms: String,
    
    // État du scan
    scan_results: Vec<ScanResult>,
    scan_progress: ScanProgress,
    is_scanning: bool,
    
    // Communication avec le thread de scan
    scan_sender: Option<Sender<()>>,
    result_receiver: Option<Receiver<ScanMessage>>,
    
    // UI state
    active_tab: usize,
    show_only_open: bool,
    
    // Port guide
    port_guide: HashMap<u16, &'static str>,
}

impl Default for PortHunterApp {
    fn default() -> Self {
        let mut port_guide = HashMap::new();
        port_guide.insert(20, "FTP (Data)");
        port_guide.insert(21, "FTP (Control)");
        port_guide.insert(22, "SSH - Secure Shell");
        port_guide.insert(23, "Telnet - Remote Login");
        port_guide.insert(25, "SMTP - Email Sending");
        port_guide.insert(53, "DNS - Domain Name System");
        port_guide.insert(80, "HTTP - Web Traffic");
        port_guide.insert(110, "POP3 - Email");
        port_guide.insert(143, "IMAP - Email");
        port_guide.insert(443, "HTTPS - Secure Web");
        port_guide.insert(993, "IMAPS - Secure IMAP");
        port_guide.insert(995, "POP3S - Secure POP3");
        port_guide.insert(3306, "MySQL - Database");
        port_guide.insert(3389, "RDP - Remote Desktop");
        port_guide.insert(135, "RPC Endpoint Mapper");
        port_guide.insert(139, "NetBIOS Session Service");
        port_guide.insert(445, "SMB/CIFS");
        port_guide.insert(1433, "Microsoft SQL Server");
        port_guide.insert(1521, "Oracle Database");
        port_guide.insert(5432, "PostgreSQL");
        port_guide.insert(6379, "Redis");
        port_guide.insert(27017, "MongoDB");

        Self {
            target_ip: "192.168.1.1".to_string(),
            start_port: "1".to_string(),
            end_port: "1024".to_string(),
            timeout_ms: "1000".to_string(),
            scan_results: Vec::new(),
            scan_progress: ScanProgress {
                current_port: 0,
                total_ports: 0,
                scanned: 0,
                open_ports: 0,
                is_scanning: false,
                hostname: String::new(),
            },
            is_scanning: false,
            scan_sender: None,
            result_receiver: None,
            active_tab: 0,
            show_only_open: true,
            port_guide,
        }
    }
}

impl PortHunterApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn start_scan(&mut self) {
        if self.is_scanning {
            return;
        }

        let start_port: u16 = self.start_port.parse().unwrap_or(1);
        let end_port: u16 = self.end_port.parse().unwrap_or(1024);
        let timeout = Duration::from_millis(self.timeout_ms.parse().unwrap_or(1000));
        let target = self.target_ip.clone();

        // Ne pas effacer les résultats précédents, juste ajouter les nouveaux
        self.is_scanning = true;
        self.scan_progress = ScanProgress {
            current_port: start_port,
            total_ports: end_port - start_port + 1,
            scanned: 0,
            open_ports: 0,
            is_scanning: true,
            hostname: String::new(),
        };

        let (tx, rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();

        self.result_receiver = Some(rx);
        self.scan_sender = Some(stop_tx);

        // Spawn scanning thread
        thread::spawn(move || {
            scan_ports_threaded(target, start_port, end_port, timeout, tx, stop_rx);
        });
    }

    fn stop_scan(&mut self) {
        if let Some(sender) = &self.scan_sender {
            let _ = sender.send(());
        }
        self.is_scanning = false;
        self.scan_progress.is_scanning = false;
    }

    fn update_from_scan_messages(&mut self) {
        if let Some(receiver) = &self.result_receiver {
            while let Ok(message) = receiver.try_recv() {
                match message {
                    ScanMessage::Progress(progress) => {
                        self.scan_progress = progress;
                    }
                    ScanMessage::Result(result) => {
                        if result.is_open {
                            // Vérifier si le port n'existe pas déjà pour éviter les doublons
                            if !self.scan_results.iter().any(|r| r.port == result.port && r.timestamp == result.timestamp) {
                                self.scan_results.push(result);
                            }
                        }
                    }
                    ScanMessage::Complete => {
                        self.is_scanning = false;
                        self.scan_progress.is_scanning = false;
                    }
                    ScanMessage::Error(error) => {
                        eprintln!("Erreur de scan: {}", error);
                        self.is_scanning = false;
                        self.scan_progress.is_scanning = false;
                    }
                }
            }
        }
    }

    fn export_results(&self) {
        if !self.scan_results.is_empty() {
            let json = serde_json::to_string_pretty(&self.scan_results).unwrap_or_default();
            let filename = format!("porthunter_{}_{}.json", 
                self.target_ip.replace(".", "_"), 
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            );
            
            if let Err(e) = std::fs::write(&filename, json) {
                eprintln!("Erreur lors de l'export: {}", e);
            } else {
                println!("Résultats exportés vers: {}", filename);
            }
        }
    }
}

impl eframe::App for PortHunterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update from background scan
        self.update_from_scan_messages();
        
        // Request repaint if scanning
        if self.is_scanning {
            ctx.request_repaint();
        }

        // Header
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.heading("🛡️ PortHunter");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.is_scanning {
                        ui.colored_label(egui::Color32::GREEN, "🔄 Scan en cours...");
                    } else {
                        ui.colored_label(egui::Color32::GRAY, "⏸️ Inactif");
                    }
                });
            });
            ui.add_space(10.0);
        });

        // Navigation tabs
        egui::TopBottomPanel::top("nav").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, 0, "🔧 Scanner");
                ui.selectable_value(&mut self.active_tab, 1, "📖 Guide des ports");
                ui.selectable_value(&mut self.active_tab, 2, "⚙️ Paramètres");
            });
            ui.separator();
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                0 => self.show_scanner_tab(ui),
                1 => self.show_port_guide_tab(ui),
                2 => self.show_settings_tab(ui),
                _ => {}
            }
        });
    }
}

impl PortHunterApp {
    fn show_scanner_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Configuration panel (left) - Plus compact
            ui.vertical(|ui| {
                ui.set_max_width(350.0); // Limiter la largeur du panneau de config
                
                ui.group(|ui| {
                    ui.heading("Configuration");
                    ui.add_space(5.0);

                    // Configuration plus compacte
                    ui.label("Adresse IP:");
                    ui.text_edit_singleline(&mut self.target_ip);
                    ui.add_space(3.0);

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Port début:");
                            ui.text_edit_singleline(&mut self.start_port);
                        });
                        ui.vertical(|ui| {
                            ui.label("Port fin:");
                            ui.text_edit_singleline(&mut self.end_port);
                        });
                    });
                    
                    ui.add_space(3.0);
                    ui.label("Timeout (ms):");
                    ui.text_edit_singleline(&mut self.timeout_ms);

                    ui.add_space(5.0);

                    if !self.scan_progress.hostname.is_empty() {
                        ui.colored_label(
                            egui::Color32::LIGHT_BLUE,
                            format!("🌐 {}", self.scan_progress.hostname)
                        );
                    }

                    ui.add_space(5.0);

                    // Control buttons - Plus compacts
                    ui.vertical(|ui| {
                        if self.is_scanning {
                            if ui.button("⏹️ Arrêter").clicked() {
                                self.stop_scan();
                            }
                        } else {
                            if ui.button("▶️ Démarrer le scan").clicked() {
                                self.start_scan();
                            }
                        }

                        ui.horizontal(|ui| {
                            if ui.button("🔄 Reset").clicked() {
                                // Bouton pour effacer VRAIMENT tous les résultats
                                self.scan_results.clear();
                                self.scan_progress = ScanProgress {
                                    current_port: 0,
                                    total_ports: 0,
                                    scanned: 0,
                                    open_ports: 0,
                                    is_scanning: false,
                                    hostname: String::new(),
                                };
                            }

                            if ui.button("🧹 Nouveau").clicked() {
                                // Bouton pour démarrer un nouveau scan en effaçant les anciens résultats
                                if !self.is_scanning {
                                    self.scan_results.clear();
                                    self.start_scan();
                                }
                            }
                        });

                        if !self.scan_results.is_empty() && ui.button("💾 Exporter").clicked() {
                            self.export_results();
                        }
                    });

                    // Progress info - Plus compact
                    if self.scan_progress.total_ports > 0 {
                        ui.add_space(5.0);
                        ui.group(|ui| {
                            ui.label("📊 Stats");
                            
                            let progress = self.scan_progress.scanned as f32 / self.scan_progress.total_ports as f32;
                            ui.add(egui::ProgressBar::new(progress)
                                .text(format!("{}/{}", self.scan_progress.scanned, self.scan_progress.total_ports)));

                            ui.horizontal(|ui| {
                                ui.small(format!("Scannés: {}", self.scan_progress.scanned));
                                ui.small(format!("Ouverts: {}", self.scan_progress.open_ports));
                            });
                        });
                    }
                });
            });

            ui.separator();

            // Results panel (right) - Prend tout l'espace restant
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading("🔍 Résultats");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.checkbox(&mut self.show_only_open, "Ports ouverts seulement");
                        });
                    });

                    ui.add_space(5.0);

                    if self.scan_results.is_empty() && !self.is_scanning {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label("🔎 Aucun résultat");
                            ui.label("Configurez une cible et démarrez le scan");
                        });
                    } else {
                        // Header avec informations de résumé - Plus compact
                        if !self.scan_results.is_empty() {
                            ui.horizontal(|ui| {
                                if self.is_scanning {
                                    ui.colored_label(egui::Color32::YELLOW, "🔄");
                                    ui.small(format!("{} ports trouvés", self.scan_results.len()));
                                } else {
                                    ui.colored_label(egui::Color32::GREEN, "✅");
                                    ui.small(format!("{} ports ouverts", self.scan_results.len()));
                                }
                                
                                ui.separator();
                                ui.colored_label(egui::Color32::LIGHT_BLUE, "🎯");
                                ui.small(&self.target_ip);
                            });
                            ui.add_space(5.0);
                        }

                        // Utiliser tout l'espace disponible pour les résultats
                        let available_height = ui.available_height() - 20.0;
                        egui::ScrollArea::vertical().max_height(available_height).show(ui, |ui| {
                            // Trier les résultats par numéro de port
                            let mut sorted_results = self.scan_results.clone();
                            sorted_results.sort_by(|a, b| a.port.cmp(&b.port));
                            
                            for (index, result) in sorted_results.iter().enumerate() {
                                ui.group(|ui| {
                                    // Header du port - Plus compact
                                    ui.horizontal(|ui| {
                                        ui.colored_label(egui::Color32::GREEN, "✅");
                                        ui.strong(format!("Port {}", result.port));
                                        ui.colored_label(egui::Color32::LIGHT_BLUE, &result.service);
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.small(format!("#{}", index + 1));
                                        });
                                    });

                                    // Informations détaillées en mode compact
                                    ui.collapsing("📋 Détails", |ui| {
                                        ui.small(format!("🏷️ {}", get_port_description(result.port)));
                                        ui.add_space(3.0);
                                        
                                        ui.horizontal(|ui| {
                                            ui.colored_label(egui::Color32::GRAY, "⏰");
                                            ui.small(&result.timestamp);
                                        });
                                        
                                        ui.add_space(3.0);
                                        ui.group(|ui| {
                                            ui.small("📋 Bannière:");
                                            let banner_text = if result.banner.trim().is_empty() {
                                                "Aucune bannière détectée".to_string()
                                            } else if result.banner.len() > 80 {
                                                format!("{}...", &result.banner[..80])
                                            } else {
                                                result.banner.clone()
                                            };
                                            ui.monospace(&banner_text);
                                        });

                                        ui.add_space(3.0);
                                        ui.group(|ui| {
                                            ui.small("⚠️ Sécurité:");
                                            ui.small(get_security_info(result.port));
                                        });
                                    });
                                });
                                ui.add_space(5.0);
                            }
                        });
                    }

                    if self.is_scanning && self.scan_results.is_empty() {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.spinner();
                            ui.label("Scan en cours...");
                            ui.small(format!("Port: {}", self.scan_progress.current_port));
                        });
                    }
                });
            });
        });
    }

    fn show_port_guide_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("📖 Guide des ports courants");
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut ports: Vec<_> = self.port_guide.iter().collect();
            ports.sort_by_key(|(port, _)| **port);

            for (port, description) in ports {
                ui.group(|ui| {
                    // Header avec numéro de port et statut sécurité
                    ui.horizontal(|ui| {
                        // Numéro de port avec couleur selon le niveau de sécurité
                        let color = match *port {
                            22 | 443 | 993 | 995 => egui::Color32::GREEN,     // Sécurisé
                            21 | 23 | 80 | 110 | 143 => egui::Color32::YELLOW, // Attention
                            3389 | 6379 => egui::Color32::RED,                // Dangereux
                            _ => egui::Color32::LIGHT_BLUE,                   // Neutre
                        };
                        
                        ui.colored_label(color, format!("Port {}", port));
                        ui.strong(*description);
                    });

                    ui.add_space(5.0);

                    // Description détaillée
                    ui.horizontal(|ui| {
                        ui.colored_label(egui::Color32::LIGHT_BLUE, "📋 Description:");
                        ui.label(get_port_description(*port));
                    });

                    ui.add_space(5.0);

                    // Informations de sécurité
                    ui.horizontal(|ui| {
                        ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "🔒 Sécurité:");
                    });
                    ui.label(get_security_info(*port));

                    ui.add_space(5.0);

                    // Usage typique
                    ui.horizontal(|ui| {
                        ui.colored_label(egui::Color32::GRAY, "💡 Usage typique:");
                        ui.label(get_typical_usage(*port));
                    });
                });
                ui.add_space(10.0);
            }
        });
    }

    fn show_settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙️ Paramètres");
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Préférences d'affichage");
            ui.checkbox(&mut self.show_only_open, "Afficher seulement les ports ouverts");
            ui.add_space(10.0);
            
            ui.label("Configuration par défaut");
            ui.horizontal(|ui| {
                ui.label("Timeout par défaut:");
                ui.text_edit_singleline(&mut self.timeout_ms);
            });
        });

        ui.add_space(20.0);

        ui.group(|ui| {
            ui.label("À propos");
            ui.label("PortHunter - Scanner de ports en Rust");
            ui.label("Version avec interface graphique native");
            ui.hyperlink_to("Code source", "https://github.com/wh0amix/PortHunter");
        });
    }
}

// Fonction de scan adaptée pour le threading
fn scan_ports_threaded(
    target: String,
    start_port: u16,
    end_port: u16,
    timeout: Duration,
    sender: Sender<ScanMessage>,
    stop_receiver: Receiver<()>,
) {
    // Résoudre le hostname
    let hostname = match target.parse::<IpAddr>() {
        Ok(ip_addr) => match lookup_addr(&ip_addr) {
            Ok(name) => name,
            Err(_) => "Non résolu".to_string(),
        },
        Err(_) => "Adresse invalide".to_string(),
    };

    let mut progress = ScanProgress {
        current_port: start_port,
        total_ports: end_port - start_port + 1,
        scanned: 0,
        open_ports: 0,
        is_scanning: true,
        hostname: hostname.clone(),
    };

    let _ = sender.send(ScanMessage::Progress(progress.clone()));

    for port in start_port..=end_port {
        // Check for stop signal
        if stop_receiver.try_recv().is_ok() {
            break;
        }

        progress.current_port = port;
        progress.scanned = port - start_port + 1;

        let _ = sender.send(ScanMessage::Progress(progress.clone()));

        if let Some(banner) = scan_port(&target, port, timeout) {
            let service = get_service_name(port);
            let result = ScanResult {
                port,
                service,
                banner,
                timestamp: {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    format!("{:02}:{:02}:{:02}", 
                        (now / 3600) % 24,
                        (now / 60) % 60, 
                        now % 60)
                },
                is_open: true,
            };

            progress.open_ports += 1;
            let _ = sender.send(ScanMessage::Result(result));
        }

        // Small delay to prevent overwhelming
        thread::sleep(Duration::from_millis(10));
    }

    let _ = sender.send(ScanMessage::Complete);
}

// Fonction de scan de port (de votre code original)
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

fn get_service_name(port: u16) -> String {
    match port {
        20 => "FTP (Data)".to_string(),
        21 => "FTP (Control)".to_string(),
        22 => "SSH - Secure Shell".to_string(),
        23 => "Telnet - Remote Login".to_string(),
        25 => "SMTP - Email Sending".to_string(),
        53 => "DNS - Domain Name System".to_string(),
        80 => "HTTP - Web Traffic".to_string(),
        110 => "POP3 - Email".to_string(),
        143 => "IMAP - Email".to_string(),
        443 => "HTTPS - Secure Web".to_string(),
        993 => "IMAPS - Secure IMAP".to_string(),
        995 => "POP3S - Secure POP3".to_string(),
        3306 => "MySQL - Database".to_string(),
        3389 => "RDP - Remote Desktop".to_string(),
        135 => "RPC Endpoint Mapper".to_string(),
        139 => "NetBIOS Session Service".to_string(),
        445 => "SMB/CIFS".to_string(),
        1433 => "Microsoft SQL Server".to_string(),
        1521 => "Oracle Database".to_string(),
        5432 => "PostgreSQL".to_string(),
        6379 => "Redis".to_string(),
        27017 => "MongoDB".to_string(),
        _ => "Service inconnu".to_string(),
    }
}

fn get_port_description(port: u16) -> String {
    match port {
        20 => "Canal de données FTP - Transfert de fichiers".to_string(),
        21 => "Canal de contrôle FTP - Commandes de transfert de fichiers".to_string(),
        22 => "Secure Shell - Accès terminal sécurisé et transfert de fichiers".to_string(),
        23 => "Telnet - Accès terminal non sécurisé (obsolète)".to_string(),
        25 => "Simple Mail Transfer Protocol - Envoi d'emails".to_string(),
        53 => "Domain Name System - Résolution de noms de domaine".to_string(),
        80 => "HyperText Transfer Protocol - Navigation web non sécurisée".to_string(),
        110 => "Post Office Protocol v3 - Réception d'emails".to_string(),
        143 => "Internet Message Access Protocol - Gestion d'emails sur serveur".to_string(),
        443 => "HTTPS - Navigation web sécurisée avec SSL/TLS".to_string(),
        993 => "IMAPS - IMAP sécurisé avec SSL/TLS".to_string(),
        995 => "POP3S - POP3 sécurisé avec SSL/TLS".to_string(),
        3306 => "MySQL Database - Base de données relationnelle".to_string(),
        3389 => "Remote Desktop Protocol - Accès bureau à distance Windows".to_string(),
        135 => "RPC Endpoint Mapper - Service RPC Windows".to_string(),
        139 => "NetBIOS Session Service - Partage de fichiers Windows".to_string(),
        445 => "SMB/CIFS - Partage de fichiers et imprimantes Windows".to_string(),
        1433 => "Microsoft SQL Server - Base de données SQL Server".to_string(),
        1521 => "Oracle Database - Base de données Oracle".to_string(),
        5432 => "PostgreSQL - Base de données PostgreSQL".to_string(),
        6379 => "Redis - Base de données en mémoire Redis".to_string(),
        27017 => "MongoDB - Base de données NoSQL MongoDB".to_string(),
        _ => "Port non standard - Vérifier la documentation du service".to_string(),
    }
}

fn get_security_info(port: u16) -> String {
    match port {
        21 => "⚠️ FTP transmet les données en clair. Préférer SFTP (port 22)".to_string(),
        22 => "✅ Protocole sécurisé. Vérifier les clés et désactiver l'auth par mot de passe".to_string(),
        23 => "🚨 TRÈS DANGEREUX - Tout est transmis en clair. À désactiver immédiatement".to_string(),
        25 => "⚠️ Peut être utilisé pour spam. Configurer l'authentification".to_string(),
        53 => "ℹ️ Service essentiel. Sécuriser contre les attaques DNS".to_string(),
        80 => "⚠️ Trafic non chiffré. Rediriger vers HTTPS (443)".to_string(),
        110 => "⚠️ Emails transmis en clair. Préférer POP3S (995) ou IMAPS (993)".to_string(),
        143 => "⚠️ Emails transmis en clair. Préférer IMAPS (993)".to_string(),
        443 => "✅ Protocole sécurisé. Vérifier les certificats SSL/TLS".to_string(),
        993 => "✅ IMAP sécurisé avec chiffrement SSL/TLS".to_string(),
        995 => "✅ POP3 sécurisé avec chiffrement SSL/TLS".to_string(),
        3306 => "⚠️ Base de données exposée. Restreindre l'accès et changer le port par défaut".to_string(),
        3389 => "🚨 RDP exposé = risque élevé. Utiliser VPN et authentification forte".to_string(),
        135 => "⚠️ Service Windows. Peut être exploité, limiter l'exposition".to_string(),
        139 => "⚠️ NetBIOS ancien protocole. Désactiver si non nécessaire".to_string(),
        445 => "⚠️ SMB vulnérable aux attaques. Mettre à jour et sécuriser".to_string(),
        1433 => "⚠️ SQL Server exposé. Changer le port et restreindre l'accès".to_string(),
        1521 => "⚠️ Oracle Database exposée. Sécuriser l'accès et changer le port".to_string(),
        5432 => "⚠️ PostgreSQL exposé. Configurer l'authentification et le chiffrement".to_string(),
        6379 => "🚨 Redis sans auth par défaut. Configurer un mot de passe immédiatement".to_string(),
        27017 => "⚠️ MongoDB exposé. Activer l'authentification et le chiffrement".to_string(),
        _ => "ℹ️ Port non standard. Identifier le service et évaluer les risques".to_string(),
    }
}

fn get_typical_usage(port: u16) -> String {
    match port {
        20 => "Serveurs FTP pour transfert de fichiers volumineux".to_string(),
        21 => "Serveurs FTP, sites web avec upload de fichiers".to_string(),
        22 => "Administration serveurs, déploiement code, tunnels sécurisés".to_string(),
        23 => "Anciens équipements réseau (routeurs, switches legacy)".to_string(),
        25 => "Serveurs mail (Postfix, Sendmail, Exchange)".to_string(),
        53 => "Serveurs DNS (BIND, PowerDNS), résolution de domaines".to_string(),
        80 => "Serveurs web (Apache, Nginx), sites internet non sécurisés".to_string(),
        110 => "Serveurs mail pour clients de messagerie traditionnels".to_string(),
        143 => "Serveurs mail (Dovecot, Exchange) pour accès IMAP".to_string(),
        443 => "Sites web sécurisés (HTTPS), APIs REST, services web".to_string(),
        993 => "Serveurs mail sécurisés pour clients de messagerie modernes".to_string(),
        995 => "Serveurs mail sécurisés pour téléchargement d'emails".to_string(),
        3306 => "Applications web avec base de données MySQL/MariaDB".to_string(),
        3389 => "Administration à distance de serveurs/postes Windows".to_string(),
        135 => "Services Windows, Active Directory, applications Microsoft".to_string(),
        139 => "Partage de fichiers en réseau local Windows".to_string(),
        445 => "Partage de fichiers/imprimantes Windows, serveurs de fichiers".to_string(),
        1433 => "Applications .NET, ERP, CRM avec base SQL Server".to_string(),
        1521 => "Applications d'entreprise, ERP Oracle, bases de données critiques".to_string(),
        5432 => "Applications web modernes, systèmes analytiques".to_string(),
        6379 => "Cache applicatif, sessions utilisateurs, données temporaires".to_string(),
        27017 => "Applications web modernes, APIs, systèmes de gestion de contenu".to_string(),
        _ => "Vérifier la documentation de l'application concernée".to_string(),
    }
}

fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("PortHunter");

    if let Some(ref fig) = figure {
        println!("{}", fig);
    }
}

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();
    
    // Si --cli est passé en argument, utiliser la version CLI
    if args.contains(&"--cli".to_string()) {
        cli::run_cli();
        return Ok(());
    }
    
    // Vérifier si l'utilisateur veut la CLI depuis l'environnement
    if let Ok(mode) = env::var("PORTHUNTER_MODE") {
        if mode == "cli" {
            cli::run_cli();
            return Ok(());
        }
    }
    
    // Sinon, lancer l'interface graphique par défaut
    print_banner();
    println!("🚀 PortHunter - Interface Graphique");
    println!("💡 Utilisez --cli pour l'interface en ligne de commande");
    println!("💡 Ou définissez PORTHUNTER_MODE=cli");
    println!("====================================");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])  // Fenêtre plus large
            .with_min_inner_size([1000.0, 700.0]), // Taille minimale adaptée
        ..Default::default()
    };

    eframe::run_native(
        "PortHunter",
        options,
        Box::new(|cc| Box::new(PortHunterApp::new(cc))),
    )
}