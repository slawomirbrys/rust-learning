use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::time::{Duration, Instant};
use futures::stream::FuturesUnordered;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
        Read the 1st argument from command line
     */
    let filename = std::env::args().nth(1);
    let filename = filename.ok_or(Error::new(ErrorKind::NotFound, "File name is missing"))?;
    println!("Loading urls from {}", filename);

    /*
        Read the file and save values to the vector
     */
    let file = File::open(&filename)?;

    let file_reader = BufReader::new(file);
    let mut urls = Vec::new();
    for url in file_reader.lines() {
        urls.push(url.unwrap());
    }

    // Debug way, ugly one
    //println!("{:?}", urls);

    for url in &urls {
        println!("Url: {}", url);
    }

    /*
        Call http request per each url
    */
    let start = Instant::now();
    let mut totals = Stats::new();
    let client = reqwest::Client::new();
    let mut requests = FuturesUnordered::new();
    for url in urls {
        requests.push(get(&client, String::from(url)));
    }

    while let Some(stats) = requests.next().await {
        println!("--------------------------------------------------------------------------------");
        // println!(
        //     "Url {} loaded in {}ms with length of {} and status code {}",
        //     "unknown",
        //     stats?.elapsed_time.as_millis(),
        //     stats?.status,
        //     stats?.content_length);
        println!("Url {} -> {:?}", "unknown", stats);

        totals.aggregate(&stats?);
    }

    /*
        Present totals
     */
    println!("=====================================================================================");
    println!("Total: {:?} ({:.2} bytes/sec)", totals, totals.bytes_per_sec().unwrap_or_default());
    println!("Main time: {:?}", start.elapsed());

    /*
        Result
     */
    Ok(())
}

#[derive(Debug)]
struct Stats {
    pub elapsed_time: Duration,
    pub content_length: usize,
    pub status: String
}

impl Stats {
    fn new() -> Self {
        Stats {
            elapsed_time: Duration::default(),
            content_length: 0,
            status: String::default()
        }
    }

    fn aggregate(&mut self, other: &Stats) {
        self.elapsed_time += other.elapsed_time;
        self.content_length += other.content_length;
        if !self.status.eq(&other.status) {
            if self.status.len() > 0 {
                self.status.push_str(", ");
            }
            self.status.push_str(other.status.as_str());
        }
    }

    fn bytes_per_sec(&self) -> Option<f64> {
        let elapsed_sec = self.elapsed_time.as_secs_f64();
        if elapsed_sec <= 0.0 {
            return None;
        }

        let bps = (self.content_length as f64)/ elapsed_sec;
        Some(bps)
    }
}

async fn get(client: &reqwest::Client, url: String) -> Result<Stats, Box<dyn std::error::Error>> {
    let start = Instant::now();
    let response = client.get(url).send().await?;
    let status_code = response.status();
    let status_code = String::from(status_code.as_str());
    //let status_code = String::from(response.status().as_str());

    let body = response.text().await?;
    let elapsed_time = start.elapsed();

    Ok(Stats {
        elapsed_time,
        content_length: body.len(),
        status: status_code
    })
}

#[cfg(test)]
mod tests {
    use crate::Stats;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn stat_default() {
        let stats = Stats::new();
        assert_eq!(stats.elapsed_time.as_secs(), 0);
    }
}