pub async fn load_csv_from_file(path: &str) -> Vec<csv::StringRecord> {
    use macroquad::prelude::load_file;
    use csv::ReaderBuilder;
    let file = load_file(path)
        .await
        .unwrap_or_else(|e| panic! {"Invalid file : {} {}", path, e});

    let csv = match std::str::from_utf8(&file) {
        Ok(v) => v,
        Err(e) => panic!("File \"{}\" have Invalid UTF-8 sequence: {} ", path, e),
    };

    let mut rdr = ReaderBuilder::new().from_reader(csv.as_bytes());
    let mut records: Vec<csv::StringRecord> = Vec::new();
    for result in rdr.records() {
        let mut record = result.expect("a CSV record");
        record.trim();
        records.push(record);
    }
    records
    
}

pub fn load_csv_from_bytes(bytes: &[u8]) -> Vec<csv::StringRecord> {
    use csv::ReaderBuilder;

    let csv = match std::str::from_utf8(bytes) {
        Ok(v) => v,
        Err(e) => panic!("From Bytes have Invalid UTF-8 sequence: {} ", e),
    };

    let mut rdr = ReaderBuilder::new().from_reader(csv.as_bytes());
    let mut records: Vec<csv::StringRecord> = Vec::new();
    for result in rdr.records() {
        let mut record = result.expect("a CSV record");
        record.trim();
        records.push(record);
    }
    records
    
}