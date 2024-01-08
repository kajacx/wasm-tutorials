mod program;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    print!("{}", program::get_text().await);
}
