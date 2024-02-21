use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
        Read the 1st argument from command line
     */
    let filename = std::env::args().nth(1);
    let filename = filename.ok_or(Error::new(ErrorKind::NotFound, "File name is missing"))?;
    println!("Loading urls from {}", filename);

    /*
        Read the file and save values to the vector
     */
    let file = match File::open(&filename) {
        Ok(file) => file,
        // Err(err) => {
        //     println!("I can't find that file, error message: {}", err);
        //     std::process::exit(1);
        // }
        Err(err) => return Err(Box::new(Error::new(err.kind(), format!("I can't find {} file", &filename))))
    };

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
    let mut totals = Stats::new();
    let client = reqwest::blocking::Client::new();
    for url in urls {
        let result = match get(&client, &url) {
            Ok(result) => result,
            Err(err) => {
                println!("Url {} couldn't be loaded due to error: {}", url, err);

                return Ok();
            }
        };
        println!("--------------------------------------------------------------------------------");
        println!("Url {} loaded in {}ms with length of {} and status code {}", url, result.elapsed_time.as_millis(), result.content_length, result.status);
        println!("Url {} -> {:?}", url, result);

        totals.aggregate(&result);
    }

    /*
        Present totals
     */
    println!("=====================================================================================");
    println!("Total: {:?} ({:.2} bytes/sec)", totals, totals.bytes_per_sec().unwrap_or_default());

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

fn get(client: &reqwest::blocking::Client, url: &str) -> Result<Stats, Box<dyn std::error::Error>> {
    let start = Instant::now();
    let response = client.get(url).send()?;
    let status_code = response.status();
    let status_code = String::from(status_code.as_str());
    //let status_code = String::from(response.status().as_str());

    let body = response.text()?;
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