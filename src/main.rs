use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::{Duration};
use clap::Clap;
use select::document::Document;
use select::predicate as p;
use telegram_bot::*;
use tokio::time::delay_for as tokio_sleep;
use rand::Rng;


#[derive(Clap, Debug, Clone)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(long)]
    web_pages_list_file: String,
    #[clap(long)]
    telegram_token: String,
    #[clap(long)]
    chat_id: i64,
    #[clap(long)]
    min_duration: u64,
    #[clap(long)]
    max_added_duration: u64
}

#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    let urls: Vec<String> = configured_urls(&opts);

    println!("{:?}", urls);

    loop {
        let vec = urls.clone();
        let opts1 = opts.clone();
        loop_on_uris(vec, opts1).await.unwrap();
    }

    Ok(())
}

async fn loop_on_uris(uris: Vec<String>, opts: Opts) -> Result<(), Error> {
    for uri in uris {
        if has_stock(&uri).await.expect("error on get") {
            print!("Stock");
            let message = format!("link {:?} has_stock!!!", uri);
            let api = Api::new(&opts.telegram_token);
            let message = SendMessage::new(ChatId::new(opts.chat_id), message);
            api.send(message).await?;
            buzz_in_raspy().await.expect("error in pi buzz");
        }

        let random: u64 = rand::thread_rng().gen_range(0, opts.max_added_duration * 1000);
        let random_duration = Duration::from_millis(random);
        let sleep =  Duration::from_secs(opts.min_duration) + random_duration;
        tokio_sleep(sleep).await;

    }
    Ok(())
}

fn configured_urls(opts: &Opts) -> Vec<String> {
    let f = File::open(&opts.web_pages_list_file).expect("Unable to open the file");
    let reader = BufReader::new(f);
    reader.lines()
        .into_iter()
        .map(Result::unwrap)
        .collect()
}


async fn has_stock(link_to_pc_comp: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(link_to_pc_comp)?.text()?;
    let document = Document::from(resp.as_str());
    let has_add_cart = document.find(p::Attr("id", "add-cart")).next().is_some();
    Ok(has_add_cart)
}

async fn buzz_in_raspy() -> Result<(), gpio_cdev::Error> {
    use gpio_cdev::{Chip, LineRequestFlags};

    let mut chip = Chip::new("/dev/gpiochip0")?;

    let h = chip
        .get_line(23)?
        .request(LineRequestFlags::OUTPUT, 0, "write-output")?;

    h.set_value(1)?;
    tokio_sleep(Duration::from_secs(1)).await;
    h.set_value(0)?;
    tokio_sleep(Duration::from_secs(1)).await;


    return Ok(());
}





