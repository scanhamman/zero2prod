use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
   match run().await {
      Ok(_s) => Ok(()),
      Err(e) => Err(e),
   }
}
 