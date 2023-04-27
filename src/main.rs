use tiny_file_server::FileServer;
use local_ip_address::local_ip;
use std::ffi::OsStr;
use unicode_segmentation::UnicodeSegmentation;

fn join_string(string: &str, left: &str, right: &str) -> String {
    format!("{}{}{}", left, string, right)
}

fn repeat(string: &str, times: usize) -> String {
    let mut output = String::new();
    for _ in 0..times {
        output.push_str(string);
    }
    output
}

fn pad(string: &str, padding: usize) -> String {
    let padding_left = (padding as f32 / 2.0).floor() as usize;
    join_string(
        string,
        &repeat(" ", padding_left),
        &repeat(" ", padding - padding_left),
    )
}

fn len(string: &str) -> usize {
    string.graphemes(true).count()
}

const BOX: [&str; 6] = ["─", "│", "╮", "╯", "╰", "╭"];

pub fn styled_print(message: String, error: bool, padding: Option<(usize, usize)>) {
    let padding = padding.unwrap_or((4, 1));
    let styled_box: Vec<_> = BOX
        .to_vec()
        .into_iter()
        .map(|str| {
            if error {
                format!("\u{1b}[31m{}\u{1b}[39m", str)
            } else {
                format!("\u{1b}[32m{}\u{1b}[39m", str)
            }
        })
        .collect();  
    let lines: Vec<_> = message.lines().collect();
    let width = {
        let mut lines = lines.clone();
        lines.sort_by(|a, b| len(b).cmp(&len(a)));
        len(lines[0])
    };
    println!(
        "{}",
        join_string(
            &repeat(&styled_box[0], width + padding.0 * 2),
            &styled_box[5],
            &styled_box[2]
        )
    );
    for _ in 0..padding.1 {
        println!(
            "{}",
            join_string(
                &repeat(" ", width + padding.0 * 2),
                &styled_box[1],
                &styled_box[1]
            )
        );
    }
    for line in lines {
        println!(
            "{}",
            join_string(
                &pad(line, width - len(line) + padding.0 * 2),
                &styled_box[1],
                &styled_box[1]
            )
        );
    }
    for _ in 0..padding.1 {
        println!(
            "{}",
            join_string(
                &repeat(" ", width + padding.0 * 2),
                &styled_box[1],
                &styled_box[1]
            )
        );
    }
    println!(
        "{}",
        join_string(
            &repeat(&styled_box[0], width + padding.0 * 2),
            &styled_box[4],
            &styled_box[3]
        )
    );
}

fn main() {
    let path = std::env::current_dir();
    let default_port = String::from("3000");
    let args: Vec<_> = std::env::args().collect();
    let port = args.get(1).unwrap_or(&default_port);
    let ip = local_ip();
    if ip.is_err() {
        println!("Unable to get private ip address.");
        return;
    }
    let ip = ip.unwrap();
    if path.is_err() {
        println!("Unable to get current directory")
    }
    let path = path.unwrap();
    let server = FileServer::http(&format!("0.0.0.0:{}", &port));
    if server.is_err() {
        println!("Unable to create server.");
        return;
    } 
    styled_print(
        format!(
            "Serving {}
Local: http://localhost:{}
Network: http://{}:{1}
Press Ctr-C to stop.",
            path.file_name().unwrap_or(OsStr::new("/")).to_str().unwrap(),
            port,
            ip.to_string()
        ),
        false,
        None,
    );
    let server = server.unwrap().run(&path);
    if server.is_err() {
        println!("Unable to listen.");
    }
}
