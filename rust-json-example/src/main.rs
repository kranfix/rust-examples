use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize, Debug)]
enum Gender {
  Masculine,
  Feminine,
  Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
  name: String,
  age: u8,
  phones: Vec<String>,

  #[serde(default = "Gender::default")]
  gender: Gender,

  addres: Option<String>,
}

impl Gender {
  fn default() -> Self {
    Self::Unknown
  }
}

fn main() {
  untyped_raw_str_example().unwrap();
  untyped_json_example().unwrap();
  typed_raw_str_example().unwrap();
  typed_json_example().unwrap();
  serilize().unwrap();
}

fn untyped_raw_str_example() -> serde_json::Result<()> {
  let data = r#"
    {
      "name": "Frank Moreno",
      "age": 27,
      "phones": [
        "+51 1 1234567",
        "+51 987654321"
      ]
    }
  "#;

  //let v: Value = serde_json::from_str::<Value>(data)?;
  let v: Value = serde_json::from_str(data)?;

  println!("Untyped raw str");
  println!("  v = {}", v);
  println!("  value = {:?}", v);
  println!(
    "  Please call {} at the number {}",
    v["name"], v["phones"][0]
  );

  Ok(())
}

fn untyped_json_example() -> serde_json::Result<()> {
  let v: Value = json!(
  {
    "name": "Frank Moreno",
    "age": 27,
    "phones": [
      "+51 1 1234567",
      "+51 987654321"
    ]
  });

  println!("Untyped json");
  println!("  v = {}", v);
  println!("  value = {:?}", v);
  println!(
    "  Please call {} at the number {}",
    v["name"], v["phones"][0]
  );

  Ok(())
}

fn typed_raw_str_example() -> serde_json::Result<()> {
  let data = r#"
    {
      "name": "Frank Moreno",
      "age": 27,
      "phones": [
        "+51 1 1234567",
        "+51 987654321"
      ]
    }
  "#;

  let u: User = serde_json::from_str(data)?;

  println!("Typed from raw str");
  println!("  u = {:?}", u);
  println!("  Please call {} at the number {}", u.name, u.phones[0]);
  Ok(())
}

fn typed_json_example() -> serde_json::Result<()> {
  let v: Value = json!(
  {
    "name": "Frank Moreno",
    "age": 27,
    "phones": [
      "+51 1 1234567",
      "+51 987654321"
    ]
  });

  let u: User = serde_json::from_value(v).unwrap();

  println!("typed json");
  println!("  u = {:?}", u);
  println!("  Please call {} at the number {}", u.name, u.phones[0]);

  Ok(())
}

fn serilize() -> serde_json::Result<()> {
  let v: Value = json!(
  {
    "name": "Frank Moreno",
    "age": 27,
    "phones": [
      "+51 1 1234567",
      "+51 987654321"
    ]
  });
  let u: User = serde_json::from_value(v).unwrap();
  println!("{}", serde_json::to_string_pretty(&u).unwrap());
  Ok(())
}
