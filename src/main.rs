use std::fs::File;
use std::io::Write;
use std::path::Path;

use landlock::{
    Access, AccessFs, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreatedAttr, RulesetStatus,
    ABI,
};

fn test_file_operations(path: &str) -> Result<(), std::io::Error> {
    println!("Testing file operations in: {}", path);
    
    // ファイルの作成と書き込み
    let file_path = Path::new(path).join("test_file.txt");
    let mut file = File::create(&file_path)?;
    file.write_all(b"Hello, Landlock!")?;
    println!("  Created and wrote to file: {:?}", file_path);


    // ファイルの読み取り
    let content = std::fs::read_to_string(&file_path)?;

    println!("  Read from file: {}", content);

    // ファイルの削除
    std::fs::remove_file(&file_path)?;
    println!("  Deleted file: {:?}", file_path);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let abi = ABI::V2;


    let mut ruleset = Ruleset::default()
        .handle_access(AccessFs::from_all(abi))?
        .create()?;

    let read_only_dirs = vec!["/etc", "/usr"];
    for dir in read_only_dirs {
        let rule = PathBeneath::new(PathFd::new(dir)?, AccessFs::ReadFile | AccessFs::ReadDir);
        ruleset = ruleset.add_rule(rule)?;
    }


    let read_write_dirs = vec!["/tmp", "/home/user/writable"];

    for dir in read_write_dirs {
        let rule = PathBeneath::new(
            PathFd::new(dir)?,
            AccessFs::ReadFile | AccessFs::ReadDir | AccessFs::WriteFile,
        );

        ruleset = ruleset.add_rule(rule)?;
    }

    let status = ruleset.restrict_self()?;

    match status.ruleset {
        RulesetStatus::FullyEnforced => {
            println!("Landlock restrictions successfully applied and fully enforced.");
        }

        RulesetStatus::PartiallyEnforced => {

            println!("Warning: Landlock restrictions partially enforced.");
        }
        RulesetStatus::NotEnforced => {
            println!("Warning: Landlock not enforced. Best-effort restrictions applied.");
        }

    }

    // テストケースの実行
    println!("\nTesting allowed directory:");
    if let Err(e) = test_file_operations("/home/user/writable") {
        println!("Error in allowed directory: {}", e);
    }

    println!("\nTesting restricted directory:");
    if let Err(e) = test_file_operations("/etc") {
        println!("Error in restricted directory: {}", e);

    }

    Ok(())
}
