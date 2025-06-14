#[cfg(test)]
mod cli_tests {
    use clap::Parser;
    use muv::cli::{Cli, Commands, CreateArgs, EnvNameArg};
    
    #[test]
    fn test_cli_parser_create() {
        let args = vec!["muv", "create", "test_env", "pytest"];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Commands::Create(args) => {
                assert_eq!(args.name, "test_env");
                assert_eq!(args.packages, vec!["pytest"]);
                assert!(args.python.is_none());
            },
            _ => panic!("Expected Create command"),
        }
    }
    
    #[test]
    fn test_cli_parser_create_with_python() {
        let args = vec!["muv", "create", "test_env", "--python", "3.10", "pytest"];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Commands::Create(args) => {
                assert_eq!(args.name, "test_env");
                assert_eq!(args.packages, vec!["pytest"]);
                assert_eq!(args.python, Some("3.10".to_string()));
            },
            _ => panic!("Expected Create command"),
        }
    }
    
    #[test]
    fn test_cli_parser_activate() {
        let args = vec!["muv", "activate", "test_env"];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Commands::Activate(args) => {
                assert_eq!(args.name, Some("test_env".to_string()));
            },
            _ => panic!("Expected Activate command"),
        }
    }
    
    #[test]
    fn test_cli_parser_list() {
        let args = vec!["muv", "list"];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Commands::List => {},
            _ => panic!("Expected List command"),
        }
    }
}