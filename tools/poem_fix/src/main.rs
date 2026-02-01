use std::{env, fs, path::Path};

fn process_file(path: &Path) {
    let Ok(content) = fs::read_to_string(path) else {
        return;
    };

    let parts: Vec<&str> = content.split("+++").collect();
    if parts.len() < 3 {
        return;
    }

    let lines: Vec<&str> = parts[2].lines().collect();
    let new_body: String = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let trimmed = line.trim_end();
            let next_non_empty = lines.get(i + 1).is_some_and(|l| !l.trim().is_empty());

            if trimmed.is_empty() || !next_non_empty {
                trimmed.to_string() // 空行 / 段末 / 孤行 → 不加
            } else {
                format!("{}  ", trimmed) // 下一行非空 → 加双空格
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let new_content = format!("+++{}+++\n{}", parts[1], new_body);

    match fs::write(path, new_content) {
        Ok(_) => println!("✓ {}", path.display()),
        Err(e) => eprintln!("✗ {} - {}", path.display(), e),
    }
}

fn process_dir(path: &Path) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            process_dir(&path);
        } else if path.extension().is_some_and(|e| e == "md") {
            process_file(&path);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("拖入文件夹或 .md 文件");
        return;
    }

    for arg in &args {
        let path = Path::new(arg);
        if path.is_dir() {
            process_dir(path);
        } else if path.extension().is_some_and(|e| e == "md") {
            process_file(path);
        }
    }

    println!("\n完成！");
}
