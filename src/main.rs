use itertools::Itertools;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};
use zip::ZipArchive;

fn unzip(path: &str) -> (Vec<String>, String) {
    let path = path.strip_suffix(".zip").unwrap_or(path);
    let (file, name) = match File::open(format!("decks/{}.zip", path)) {
        Ok(file) => (file, path.to_owned()),
        Err(_) => {
            let paths = fs::read_dir("decks").unwrap();

            let mut name = None;
            let mut file = None;
            for path in paths {
                let path = path.unwrap().path();
                if path.extension() == Some(OsStr::new("zip")) {
                    let name_suffix = path.file_name().unwrap().to_str().unwrap();
                    name = Some(
                        name_suffix
                            .strip_suffix(".zip")
                            .unwrap_or(name_suffix)
                            .to_string(),
                    );
                    file = Some(File::open(path).unwrap());
                    break;
                }
            }

            (file.unwrap(), name.unwrap())
        }
    };
    let mut archive = ZipArchive::new(file).unwrap();

    let mut paths = Vec::new();
    let mut total_size = 0;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let file_name = match file.enclosed_name() {
            Some(path) => Some(path.to_owned()),
            None => None,
        };
        if let Some(file_name) = file_name {
            let out_path: PathBuf = [
                "output",
                &format!("{}-images", name),
                &file_name.display().to_string(),
            ]
            .into_iter()
            .collect();

            println!("Extracted {}", file_name.display());
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }

            let mut outfile = File::create(&out_path).unwrap();

            io::copy(&mut file, &mut outfile).unwrap();

            total_size += file.size();
            paths.push(file_name.display().to_string());
        }
    }

    println!(
        "\nExtracted {} files with total size of {} megabytes\n",
        paths.len(),
        f64::round(total_size as f64 / 1024.0 / 1024.0)
    );

    (paths, name)
}

fn generate_html(file_names: &[String], name: &str) {
    let html_begin = format!("<!DOCTYPE html>\n<html lang=\"en\">\n\t<head>\n\t\t<meta charset=\"UTF-8\">\n\t\t<title>{}</title>\n\t\t<style>\n\t\t\t@media print {{\n\t\t\t\tdiv {{ page-break-before: always; }}\n\t\t\t}}\n\t\t</style>\n\t</head>\n\t<body>", name);
    let html_end = "\t</body>\n</html>";

    let body = file_names.iter().chunks(9).into_iter().enumerate().map(|(i, chunk)|  {
        let div_begin = format!("\t\t<div style=\"background:black; width: 7.5in; height: 10.5in; position:absolute; top: {}in; left: 0.5in;\">", 0.25 + i as f32 * 11.0);
        let div_end = "\t\t</div>";
        let images = (0..9).zip(chunk.map(|x| Some(x)).chain([None].into_iter().cycle())).map(|(j, file_name)| {
            if let Some(file_name) = file_name {
                format!("\n\t\t\t<img src=\"{}-images\\{}\" alt=\"{}\" width=\"240\" height=\"336\" style=\"width:2.5in; height:3.5in; position:absolute; top:{}in; left:{}in;\">", name, file_name, file_name, (j / 3) as f32 * 3.5, (j % 3) as f32 * 2.5)
            }
            else {
                format!("\n\t\t\t<div style=\"background:white; width:2.5in; height:3.5in; position:absolute; top:{}in; left:{}in;\"></div>", (j / 3) as f32 * 3.5, (j % 3) as f32 * 2.5)
            }
        }).join("\n");
        format!("{}\n{}\n{}", div_begin, images, div_end)
    })
    .join("\n\n");

    let html = format!("{}\n{}\n{}", html_begin, body, html_end);

    let mut file = File::create(format!("output\\{}.html", name)).unwrap();
    file.write(html.as_bytes()).unwrap();

    println!("Generated HTML file");
}

fn main() {
    let name = "deck-name";

    let (file_names, name) = unzip(name);
    generate_html(&file_names, &name);
}
