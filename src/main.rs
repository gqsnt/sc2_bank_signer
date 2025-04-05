use clap::Parser;
use log::{error, info, LevelFilter};
use sc2_bank_signer::bank_parser::BankParser;
use sc2_bank_signer::{AppResult, Args};


fn setup_logger() {

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();
}


fn run_app() -> AppResult<()>{
    let args = Args::parse();
    setup_logger();
    // Create the parser (which also computes the signature)
    let bank_parser = BankParser::new(&args)?;

    println!("{}", bank_parser.bank_path);

    let matches = bank_parser.compare_signature();

    // Handle writing back to file
    if args.write {
        if matches {
            info!("Signature already matches. No replacement needed.");
        } else {
            info!("Signature differs, attempting replacement...");
            bank_parser.replace_signature()?;
            info!("Bank file signature updated successfully.");
        }
    } else if !matches {
        info!("Signature does not match. Run with --write (-w) flag to replace the signature in the file.");
    } else {
        info!("Signature matches. No action requested.");
    }
    Ok(())
}

fn main(){
    match run_app() {
        Ok(_) => {
            info!("Operation completed successfully.");
        }
        Err(err) => {

            error!("{}", err);
            std::process::exit(1);
        }
    }
}
