#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mir_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mir_msgs__msg__BatteryCurrents() -> *const std::ffi::c_void;
}

#[link(name = "mir_msgs__rosidl_generator_c")]
extern "C" {
    fn mir_msgs__msg__BatteryCurrents__init(msg: *mut BatteryCurrents) -> bool;
    fn mir_msgs__msg__BatteryCurrents__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BatteryCurrents>, size: usize) -> bool;
    fn mir_msgs__msg__BatteryCurrents__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BatteryCurrents>);
    fn mir_msgs__msg__BatteryCurrents__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BatteryCurrents>, out_seq: *mut rosidl_runtime_rs::Sequence<BatteryCurrents>) -> bool;
}

// Corresponds to mir_msgs__msg__BatteryCurrents
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mir_msgs__msg__BatteryCurrents__init(&mut msg as *mut _) {
        panic!("Call to mir_msgs__msg__BatteryCurrents__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BatteryCurrents {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__msg__BatteryCurrents__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__msg__BatteryCurrents__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__msg__BatteryCurrents__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BatteryCurrents {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BatteryCurrents where Self: Sized {
  const TYPE_NAME: &'static str = "mir_msgs/msg/BatteryCurrents";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mir_msgs__msg__BatteryCurrents() }
  }
}


#[link(name = "mir_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mir_msgs__msg__BMSData() -> *const std::ffi::c_void;
}

#[link(name = "mir_msgs__rosidl_generator_c")]
extern "C" {
    fn mir_msgs__msg__BMSData__init(msg: *mut BMSData) -> bool;
    fn mir_msgs__msg__BMSData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BMSData>, size: usize) -> bool;
    fn mir_msgs__msg__BMSData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BMSData>);
    fn mir_msgs__msg__BMSData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BMSData>, out_seq: *mut rosidl_runtime_rs::Sequence<BMSData>) -> bool;
}

// Corresponds to mir_msgs__msg__BMSData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    pub cell_voltage: rosidl_runtime_rs::Sequence<u32>,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mir_msgs__msg__BMSData__init(&mut msg as *mut _) {
        panic!("Call to mir_msgs__msg__BMSData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BMSData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__msg__BMSData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__msg__BMSData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__msg__BMSData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BMSData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BMSData where Self: Sized {
  const TYPE_NAME: &'static str = "mir_msgs/msg/BMSData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mir_msgs__msg__BMSData() }
  }
}


