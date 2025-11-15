use clap::Parser;
use time_checker::cli::{Cli, Commands};

#[test]
fn test_cli_start_command() {
    let args = vec!["time-checker", "start", "プログラミング"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Start { task, note } => {
            assert_eq!(task, "プログラミング");
            assert_eq!(note, None);
        }
        _ => panic!("Expected Start command"),
    }
}

#[test]
fn test_cli_start_command_with_note() {
    let args = vec!["time-checker", "start", "ドキュメント作成", "--note", "設計書更新"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Start { task, note } => {
            assert_eq!(task, "ドキュメント作成");
            assert_eq!(note, Some("設計書更新".to_string()));
        }
        _ => panic!("Expected Start command"),
    }
}

#[test]
fn test_cli_start_command_with_note_short() {
    let args = vec!["time-checker", "start", "会議", "-n", "週次定例"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Start { task, note } => {
            assert_eq!(task, "会議");
            assert_eq!(note, Some("週次定例".to_string()));
        }
        _ => panic!("Expected Start command"),
    }
}

#[test]
fn test_cli_stop_command() {
    let args = vec!["time-checker", "stop"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Stop => {}
        _ => panic!("Expected Stop command"),
    }
}

#[test]
fn test_cli_status_command() {
    let args = vec!["time-checker", "status"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Status => {}
        _ => panic!("Expected Status command"),
    }
}

#[test]
fn test_cli_show_command_default() {
    let args = vec!["time-checker", "show"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Show { period } => {
            assert_eq!(period, "today");
        }
        _ => panic!("Expected Show command"),
    }
}

#[test]
fn test_cli_show_command_with_period() {
    let args = vec!["time-checker", "show", "week"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Show { period } => {
            assert_eq!(period, "week");
        }
        _ => panic!("Expected Show command"),
    }
}

#[test]
fn test_cli_task_with_spaces() {
    let args = vec!["time-checker", "start", "バグ修正 issue #123"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Start { task, note } => {
            assert_eq!(task, "バグ修正 issue #123");
            assert_eq!(note, None);
        }
        _ => panic!("Expected Start command"),
    }
}
