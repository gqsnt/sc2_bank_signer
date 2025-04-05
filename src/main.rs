use clap::Parser;
use sc2_bank_signer::{AppResult, Args};
use sc2_bank_signer::bank_parser::BankParser;

fn main() -> AppResult<()> {
    let args = Args::parse();

    // Create the parser (which also computes the signature)
    let bank_parser =  BankParser::new(&args)?;
    println!("{}", bank_parser.bank_path);
    let matches = bank_parser.compare_signature();

    // Only attempt to write if the flag is set
    if args.write {
        if matches {
            println!("Signature already matches. No replacement needed.");
        } else {
            // Call the replace function to overwrite the file
            match bank_parser.replace_signature() {
                Ok(_) => println!("Bank file signature updated successfully."),
                Err(e) => {
                    eprintln!("Failed to replace signature: {}", e);
                    // Decide if you want to return the error or just print it
                    return Err(e);
                }
            }
        }
    } else {
        println!("Run with --write (-w) flag to replace the signature in the file.");
    }
    Ok(())
}
