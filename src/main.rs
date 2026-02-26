use clap::Parser;
use colored::Colorize;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::path::{Path, PathBuf};
use rayon::prelude::*;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = ".")]
    path: String,
    #[arg(short, long)]
    recursive: bool,
}

fn main() {
    let args = Args::parse();
    let root_path = Path::new(&args.path);

    let mut files: Vec<PathBuf> = Vec::new();
    let max_depth = if args.recursive { usize::MAX } else { 1 };

    for entry in WalkDir::new(root_path).max_depth(max_depth).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }

    println!("{} Memproses {} file...\n", "INFO:".blue().bold(), files.len());

    files.par_iter().for_each(|file_path| {
        advanced_detect(file_path);
    });
}

fn advanced_detect(path: &Path) {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown");

    // 1. CEK FILE SPESIFIK (Nama File Persis)
    if let Some(special_type) = check_special_files(file_name) {
        print_res(file_name, "CONFIG", special_type);
        return;
    }

    // 2. CEK MAGIC NUMBER (Binary/ISO/Image/Archive)
    if let Ok(Some(info)) = infer::get_from_path(path) {
        print_res(file_name, &info.extension().to_uppercase(), info.mime_type());
        return;
    }

    // 3. CEK SHEBANG (Script .sh, .py, dsb)
    if let Some(script_type) = check_shebang(path) {
        print_res(file_name, "SCRIPT", script_type);
        return;
    }

    // 4. CEK ISO MANUAL
    if is_iso_file(path) {
        print_res(file_name, "ISO", "application/x-iso9660-image");
        return;
    }

    // 5. TEBAKAN BERDASARKAN EKSTENSI
    let guess = mime_guess::from_path(path).first_raw();
    if let Some(mime) = guess {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("FILE");
        print_res(file_name, &ext.to_uppercase(), mime);
        return;
    }

    // 6. INSPEKSI ISI MENTAH (Fallback terakhir)
    inspect_raw(path, file_name);
}

// Logika baru untuk file khusus seperti .gitignore
fn check_special_files(name: &str) -> Option<&'static str> {
    match name {
        ".gitignore" => Some("text/x-git-ignore"),
        ".env" => Some("text/x-dotenv"),
        "Dockerfile" => Some("text/x-dockerfile"),
        "Makefile" => Some("text/x-makefile"),
        "Cargo.toml" => Some("text/x-rust-config"),
        _ => None,
    }
}

fn check_shebang(path: &Path) -> Option<&'static str> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);

    if first_line.starts_with("#!") {
        if first_line.contains("sh") || first_line.contains("bash") { return Some("text/x-shellscript"); }
        if first_line.contains("python") { return Some("text/x-python"); }
        if first_line.contains("node") { return Some("application/javascript"); }
        if first_line.contains("php") { return Some("text/x-php"); }
    }
    None
}

fn is_iso_file(path: &Path) -> bool {
    let mut file = match File::open(path) { Ok(f) => f, Err(_) => return false };
    let mut buffer = [0; 5];
    // Offset standard ISO 9660
    if std::io::Seek::seek(&mut file, std::io::SeekFrom::Start(32769)).is_ok() {
        if file.read_exact(&mut buffer).is_ok() {
            return &buffer == b"CD001";
        }
    }
    false
}

fn inspect_raw(path: &Path, name: &str) {
    let mut file = match File::open(path) { Ok(f) => f, Err(_) => return };
    let mut buffer = [0; 1024];
    let n = file.read(&mut buffer).unwrap_or(0);
    
    match content_inspector::inspect(&buffer[..n]) {
        content_inspector::ContentType::BINARY => print_res(name, "BIN", "application/octet-stream"),
        content_inspector::ContentType::UTF_8 => print_res(name, "TXT", "text/plain"),
        _ => print_res(name, "DATA", "unknown/raw-data"),
    }
}

fn print_res(name: &str, ext: &str, mime: &str) {
    let color = match mime {
        m if m.contains("git-ignore") || m.contains("dotenv") => colored::Color::Yellow,
        m if m.contains("shellscript") || m.contains("x-executable") => colored::Color::Red,
        m if m.contains("image/") => colored::Color::Green,
        m if m.contains("iso") => colored::Color::Magenta,
        m if m.contains("text/") => colored::Color::Cyan,
        _ => colored::Color::White,
    };
    
    println!(
        "{:<30} | {:<10} | {}",
        truncate(name, 28).white(),
        ext.bold().color(color),
        mime.italic().dimmed()
    );
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max { format!("{}..", &s[0..max - 2]) } else { s.to_string() }
}
