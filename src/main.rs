use clap::Parser;
use sc2_bank_signer::{AppResult, Args};
use sc2_bank_signer::bank_parser::BankParser;

fn main() -> AppResult<()> {
    let args = Args::parse();
    let bank_parser =  BankParser::new(args.bank_path,args.name)?;
    bank_parser.compare_signature();
    Ok(())
}
