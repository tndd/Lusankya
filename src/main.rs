
use futures::executor::block_on;
use tokio::time::{sleep, Duration};


async fn learn_song() -> String {
    let delay = Duration::from_secs(5);
    println!("[learned song] in {} seconds", delay.as_secs());
    sleep(delay).await;
    let song = "Who's the leader of the club~";
    println!("[learned song] done");
    song.to_string()
}

async fn sing_song(song: String) {
    println!("[singing] {}", song);
    let delay = Duration::from_secs(1);
    sleep(delay).await;
    println!("[singing] done");
}

async fn dance() {
    println!("[dance] electrical parade!");
    let delay = Duration::from_secs(1);
    sleep(delay).await;
    println!("[dance] done");
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}
#[tokio::main]
async fn main() {
    block_on(async_main());
}
