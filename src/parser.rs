use std::{process::Command, collections::HashMap, str::FromStr};
use serde::Serialize;
use substring::Substring;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct PowerStats {
  last_power_event: String,
  rating_power_watt: u32,
  output_voltage: u32,
  model_name: String,
  battery_capacity: f32,
  state: String,
  power_supply_by: String,
  test_result: String,
  remaining_runtime_min: u32,
  line_interaction: String,
  firmware_number: String,
  utility_voltage: u32,
  load_watt: u32,
  load_watt_percent: f32,
  rating_voltage: u32,
}

impl PowerStats {
  pub fn new() -> Result<Self, String> {
    let output = Command::new("pwrstat").arg("-status").output().map_err(|x| x.to_string())?;

    if output.status.success() {
      let lines = String::from_utf8(output.stdout).map_err(|x| x.to_string())?;

      let lines = lines
        .split('\n')
        .into_iter()
        .map(|x| x.replace('\t', ""))
        .filter(|x| !x.is_empty())
        .filter(|x| x.chars().into_iter().filter(|x| x.eq(&'.')).collect::<Vec<_>>().len() > 3)
        .map(|x| (
          x.substring(
            0, 
            x.find(|y: char| y.eq(&'.')).unwrap()
          ).to_string(), 
          x.substring(
            30, 
            x.len()
          ).to_string()
        ))
        .collect::<HashMap<_, _>>();
  
      Ok(PowerStats {
        last_power_event: lines["Last Power Event"].clone(),
        rating_power_watt: Self::first_term(lines["Rating Power"].clone()),
        output_voltage: Self::first_term(lines["Output Voltage"].clone()),
        model_name: lines["Model Name"].clone(),
        battery_capacity: Self::first_term::<f32>(lines["Battery Capacity"].clone()),
        state: lines["State"].clone(),
        power_supply_by: lines["Power Supply by"].clone(),
        test_result: Self::first_term(lines["Test Result"].clone()),
        remaining_runtime_min: Self::first_term(lines["Remaining Runtime"].clone()),
        line_interaction: lines["Line Interaction"].clone(),
        firmware_number: lines["Firmware Number"].clone(),
        utility_voltage: Self::first_term(lines["Utility Voltage"].clone()),
        load_watt: Self::first_term(lines["Load"].clone()),
        load_watt_percent: Self::load_watt_percent(lines["Load"].clone()),
        rating_voltage: Self::first_term(lines["Rating Voltage"].clone()),
    })
    } else {
      Err("Permission Denied".to_string())
    }
  }

  fn first_term<T: FromStr>(str: String) -> T where <T as FromStr>::Err: Debug {
    str.split(' ').into_iter().next().unwrap().to_string().parse::<T>().unwrap()
  }

  fn load_watt_percent(str: String) -> f32 {
    str.split(' ').collect::<Vec<_>>()[1].split('(').collect::<Vec<_>>()[1].parse::<f32>().unwrap() / 100.0
  }
}
