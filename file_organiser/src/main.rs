use std::fs::{self, DirEntry};
use std::io::{self, Result, Write};
use std::path::{self, Path, PathBuf};


// a file organising app like
// images any kind will move to Images/ , similarly videos to Videos/ and any kind of documents to Documents/ and other files to Other/ folder

// road map
// 1. Take the directory input from the user
// create folders named as Images, Videos, Documents, Others
// Then after this I will iterate over the files only and move them to appropriate folders and after that 
// if any of the folder remains empty will delete it before exiting

enum FileType {
    Images,
    Videos,
    Documents,
    Others,
}

impl FileType {
    fn folder_name(&self) -> &str {
        match self {
            FileType::Images => "Images",
            FileType::Videos => "Videos",
            FileType::Documents => "Documents",
            FileType::Others => "Others",
        }
    }

    fn from_extension(extension: Option<&str>) -> FileType {
        match extension.map(|value| value.to_ascii_lowercase()) {
            Some(extension) if matches!(extension.as_str(), "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg") => FileType::Images,
            Some(extension) if matches!(extension.as_str(), "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm") => FileType::Videos,
            Some(extension) if matches!(extension.as_str(), "pdf" | "doc" | "docx" | "txt" | "rtf" | "ppt" | "pptx" | "xls" | "xlsx" | "csv" | "odt") => FileType::Documents,
            _ => FileType::Others,
        }
    }
}

fn create_folders(dir: &Path) -> Result<()> {
    for folder in ["Images", "Videos", "Documents", "Others"] {
        fs::create_dir(dir.join(folder))?;
    }
    Ok(())
}

fn take_input() -> Result<PathBuf> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    let mut dir = String::new();
    print!("Enter the directory path to organise: ");
    stdout.flush().expect("Failed to flush output");
    stdin.read_line(&mut dir).expect("Failed to read directory name");

    let dir = Path::new(&dir);

    let abs_path = path::absolute(dir);
    println!("Directory inputed: {:?}", abs_path);

    abs_path
}

fn find_folder_to_go(extension: Option<&str>) -> FileType {
    FileType::from_extension(extension)
}

fn move_file(entry: &DirEntry, target_dir: &Path) -> Result<()> {
    let file_type = entry.file_type()?;

    if !file_type.is_file() {
        return Ok(());
    }

    let file_path = entry.path();
    let extension = file_path.extension().and_then(|value| value.to_str());
    let folder = find_folder_to_go(extension);
    let destination = target_dir.join(folder.folder_name()).join(entry.file_name());

    fs::rename(file_path, destination)?;
    Ok(())
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let dir_result = take_input()?;

    create_folders(&dir_result)?;

    let entries = fs::read_dir(&dir_result)?;

    for entry in entries {
        let entry = entry?;
        move_file(&entry, &dir_result)?;
    }

    Ok(())
}
