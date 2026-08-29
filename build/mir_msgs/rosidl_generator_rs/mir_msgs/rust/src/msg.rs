#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mir_msgs__msg__BatteryCurrents

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BatteryCurrents {

    // This member is not documented.
    #[allow(missing_docs)]
    pub battery1_current: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub battery2_current: f64,

}



impl Default for BatteryCurrents {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BatteryCurrents::default())
  }
}

impl rosidl_runtime_rs::Message for BatteryCurrents {
  type RmwMsg = super::msg::rmw::BatteryCurrents;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        battery1_current: msg.battery1_current,
        battery2_current: msg.battery2_current,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      battery1_current: msg.battery1_current,
      battery2_current: msg.battery2_current,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      battery1_current: msg.battery1_current,
      battery2_current: msg.battery2_current,
    }
  }
}


// Corresponds to mir_msgs__msg__BMSData

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BMSData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pack_voltage: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub charge_current: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub discharge_current: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state_of_charge: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub remaining_time_to_full_charge: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub remaining_capacity: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state_of_health: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status_flags: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: i32,

    /// In Mk2 robots and above the BMS provides data for 8 battery cells. MiR500 robots have BMS for 13 battery cells
    pub cell_voltage: Vec<u32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub last_battery_msg_time: f64,

}

impl BMSData {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DISCHARGING: i32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CHARGING: i32 = 2;

}


impl Default for BMSData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BMSData::default())
  }
}

impl rosidl_runtime_rs::Message for BMSData {
  type RmwMsg = super::msg::rmw::BMSData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pack_voltage: msg.pack_voltage,
        charge_current: msg.charge_current,
        discharge_current: msg.discharge_current,
        state_of_charge: msg.state_of_charge,
        remaining_time_to_full_charge: msg.remaining_time_to_full_charge,
        remaining_capacity: msg.remaining_capacity,
        state_of_health: msg.state_of_health,
        status_flags: msg.status_flags,
        temperature: msg.temperature,
        cell_voltage: msg.cell_voltage.into(),
        last_battery_msg_time: msg.last_battery_msg_time,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      pack_voltage: msg.pack_voltage,
      charge_current: msg.charge_current,
      discharge_current: msg.discharge_current,
      state_of_charge: msg.state_of_charge,
      remaining_time_to_full_charge: msg.remaining_time_to_full_charge,
      remaining_capacity: msg.remaining_capacity,
      state_of_health: msg.state_of_health,
      status_flags: msg.status_flags,
      temperature: msg.temperature,
        cell_voltage: msg.cell_voltage.as_slice().into(),
      last_battery_msg_time: msg.last_battery_msg_time,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pack_voltage: msg.pack_voltage,
      charge_current: msg.charge_current,
      discharge_current: msg.discharge_current,
      state_of_charge: msg.state_of_charge,
      remaining_time_to_full_charge: msg.remaining_time_to_full_charge,
      remaining_capacity: msg.remaining_capacity,
      state_of_health: msg.state_of_health,
      status_flags: msg.status_flags,
      temperature: msg.temperature,
      cell_voltage: msg.cell_voltage
          .into_iter()
          .collect(),
      last_battery_msg_time: msg.last_battery_msg_time,
    }
  }
}


