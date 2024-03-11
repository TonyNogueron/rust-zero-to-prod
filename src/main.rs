use zero_to_prod_example::run;

#[tokio::main] // <- this is the same as tokio::main
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
