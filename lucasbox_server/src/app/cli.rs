use clap::Parser;

#[derive(Parser)]
#[command(name = "LucasBox")]
#[command(author, version, about)]
pub struct Cli {
  /// URL to a postgres database.
  /// The URL format is: postgresql://[user[:password]@][netloc][:port][/dbname][?param1=value1&...]
  #[arg(long, env)]
  pub database_url: String,
}
