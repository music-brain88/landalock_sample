use std::process::Command;

use landlock::{
    Access, AccessFs, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreatedAttr, RulesetStatus,
    ABI,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Landlockの利用可能なABIを取得
    let abi = ABI::V2;

    // ルールセットの作成
    let mut ruleset = Ruleset::default()
        .handle_access(AccessFs::from_all(abi))?
        .create()?;

    // 読み取り専用ディレクトリの設定
    let read_only_dirs = vec!["/etc", "/usr"];
    for dir in read_only_dirs {
        let rule = PathBeneath::new(PathFd::new(dir)?, AccessFs::ReadFile | AccessFs::ReadDir);
        ruleset = ruleset.add_rule(rule)?;
    }

    // 読み書き可能ディレクトリの設定
    let read_write_dirs = vec!["/tmp", "/home/user/writable"];
    for dir in read_write_dirs {
        let rule = PathBeneath::new(
            PathFd::new(dir)?,
            AccessFs::ReadFile | AccessFs::ReadDir | AccessFs::WriteFile,
        );
        ruleset = ruleset.add_rule(rule)?;
    }

    // Landlock制限の適用
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

    // スクリプトの実行（例として echo コマンドを使用）
    let output = Command::new("echo")
        .arg("Script execution simulated")
        .output()?;

    println!("Script output: {}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
