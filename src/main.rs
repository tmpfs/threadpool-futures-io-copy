use anyhow::Result;
use threadpool::ThreadPool;
use tokio::fs;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[tokio::main(flavor = "current_thread")]
async fn copy_file_current_thread() -> Result<()> {
    let passphrase = secrecy::SecretString::new("super-secret".to_owned());
    let input = fs::File::open("Cargo.toml").await.unwrap();
    let encryptor = age::Encryptor::with_user_passphrase(passphrase);

    let mut encrypted = Vec::new();
    let mut writer = encryptor.wrap_async_output(&mut encrypted).await?;
    futures::io::copy(&mut input.compat(), &mut writer).await?;
    writer.finish()?;
    fs::write("Copy.toml", encrypted).await?;
    Ok(())
}

fn main() {
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    pool.execute(move || {
        let _ = copy_file_current_thread();
    });
    pool.join();
}
