use csv::ReaderBuilder;
use std::error::Error;
use std::process;

fn example(data: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_reader(data.as_bytes());

    let numofrecords = rdr.records().count();
    println!("Number of records = {:?}", numofrecords);
    /*
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the error here
            let record = result?;
            println!("{:?}", record);
        }
    */
    Ok(())
}

fn main() {
    //let data: &'static str = "Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning: 'public.iox.h2o_xtemperature' not found\", details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Wed, 20 Jul 2022 19:08:52 GMT\", \"content-length\": \"0\"} }";
    let data: &'static str = "bottom_degrees,location,state,surface_degrees,time\n51.3,coyote_creek,CA,55.1,1970-01-01T00:00:01.568756160\n50.9,coyote_creek,CA,50.2,1970-01-01T00:00:01.600756160\n50.4,santa_monica,CA,65.2,1970-01-01T00:00:01.568756160\n49.2,santa_monica,CA,63.6,1970-01-01T00:00:01.600756160\n40.2,puget_sound,WA,55.8,1970-01-01T00:00:01.568756160\n40.1,puget_sound,WA,54.7,1970-01-01T00:00:01.600756160\n";
    if let Err(err) = example(data) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
