use crazyflie_lib::Crazyflie;
use crazyflie_lib::Value;
use crazyflie_lib::ValueType;

pub async fn list(cf: &Crazyflie) -> Result<(), Box<dyn std::error::Error>> {
  
    println!("{: <30} | {: <6} | {: <6}", "Name", "Access", "Value");
    println!("{0:-<30}-|-{0:-<6}-|-{0:-<6}", "");

    for name in cf.param.names() {
        let value: crazyflie_lib::Value = cf.param.get(&name).await?;
        let writable = if cf.param.is_writable(&name)? {
            "RW"
        } else {
            "RO"
        };

        println!("{: <30} | {: ^6} | {:?}", name, writable, value);
    }

    Ok(())
}

pub async fn get(cf: &Crazyflie, name: &str) -> Result<(), Box<dyn std::error::Error>> {

  let value:Value = cf.param.get(name).await?;

  println!("{}: {:?}", name, value);

  Ok(())
}

pub async fn set(cf: &Crazyflie, name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {

  match cf.param.get_type(name) {
    Ok(ValueType::U8) => {
      let value:u8 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::U16) => {
      let value:u16 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::U32) => {
      let value:u32 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::U64) => {
      let value:u64 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::I8) => {
      let value:i8 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::I16) => {
      let value:i16 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::I32) => {
      let value:i32 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::I64) => {
      let value:i64 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::F16) => {
      let value:f32 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::F32) => {
      let value:f32 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    Ok(ValueType::F64) => {
      let value:f64 = value.parse().expect("Invalid value");
      cf.param.set(name, value).await?;
    },
    _ => {
      println!("Invalid value type");
    }
  }

  Ok(())
}

