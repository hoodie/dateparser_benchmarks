extern crate nom;
extern crate iso8601;
use iso8601::datetime;

fn main(){

    let tests = [
        "2015-10-24T16:30:48+00:00",
        "2015-10-24T16:30:48Z",
        "20151024T163048Z",
        "2015-W43T16:30:48Z",
        "2015-W43-6T16:30:48Z",
        "2015-297T16:30:48Z",
    ];

    for date in tests.iter(){
        let parsed_nom = datetime(date.as_bytes());
        println!("{}\n ->      {:?}\n", date, parsed_nom);
    }

}


