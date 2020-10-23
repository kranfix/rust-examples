use serde::{Deserialize, Serialize};
use serde_cbor::Value as Cbor;
use serde_json::Value as Json;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Mascot {
  name: String,
  species: String,
  year_of_birth: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
  let ferris = Mascot {
    name: "Ferris".to_owned(),
    species: "crab".to_owned(),
    year_of_birth: 2015,
  };

  {
    let ferris_file = File::create("examples/ferris.cbor")?;
    // Write Ferris to the given file.
    // Instead of a file you can use any type that implements `io::Write`
    // like a HTTP body, database connection etc.
    serde_cbor::to_writer(ferris_file, &ferris)?;
    let bytes = serde_cbor::to_vec(&ferris).unwrap();
    let ferris2: Mascot = serde_cbor::from_slice(&bytes).unwrap();
    println!("ferris2: {:?}", ferris2);

    let s = bytes.iter().map(|&c| c as char).collect::<String>();
    println!("ferris string: {}", s);
  }

  // Load Tux from a file.
  // Serde CBOR performs roundtrip serialization meaning that
  // the data will not change in any way.
  let tux: Mascot = {
    let tux_file = File::open("examples/ferris.cbor")?;
    serde_cbor::from_reader(tux_file)?
  };
  let j_tux: Json = {
    let tux_file = File::open("examples/ferris.cbor")?;
    serde_cbor::from_reader(tux_file)?
  };

  println!("{:?}", tux);
  println!("{:?}", j_tux);
  // prints: Mascot { name: "Tux", species: "penguin", year_of_birth: 1996 }

  {
    let ferris_file = File::create("examples/ferris2.cbor")?;
    // Write Ferris to the given file.
    // Instead of a file you can use any type that implements `io::Write`
    // like a HTTP body, database connection etc.
    serde_cbor::to_writer(ferris_file, &ferris)?;
  }
  let j_tux: Json = {
    let tux_file = File::open("examples/ferris2.cbor")?;
    serde_cbor::from_reader(tux_file)?
  };
  println!("{:?}", j_tux);

  let c_tux: Cbor = {
    let tux_file = File::open("examples/ferris2.cbor")?;
    serde_cbor::from_reader(tux_file)?
  };
  println!("{:?}", c_tux);

  Ok(())
}
