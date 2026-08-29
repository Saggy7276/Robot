#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "mir_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mir_msgs__srv__ExecMission_Request() -> *const std::ffi::c_void;
}

#[link(name = "mir_msgs__rosidl_generator_c")]
extern "C" {
    fn mir_msgs__srv__ExecMission_Request__init(msg: *mut ExecMission_Request) -> bool;
    fn mir_msgs__srv__ExecMission_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecMission_Request>, size: usize) -> bool;
    fn mir_msgs__srv__ExecMission_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecMission_Request>);
    fn mir_msgs__srv__ExecMission_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecMission_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecMission_Request>) -> bool;
}

// Corresponds to mir_msgs__srv__ExecMission_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecMission_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mission_name: rosidl_runtime_rs::String,

}



impl Default for ExecMission_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mir_msgs__srv__ExecMission_Request__init(&mut msg as *mut _) {
        panic!("Call to mir_msgs__srv__ExecMission_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecMission_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__srv__ExecMission_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__srv__ExecMission_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__srv__ExecMission_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecMission_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecMission_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mir_msgs/srv/ExecMission_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mir_msgs__srv__ExecMission_Request() }
  }
}


#[link(name = "mir_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mir_msgs__srv__ExecMission_Response() -> *const std::ffi::c_void;
}

#[link(name = "mir_msgs__rosidl_generator_c")]
extern "C" {
    fn mir_msgs__srv__ExecMission_Response__init(msg: *mut ExecMission_Response) -> bool;
    fn mir_msgs__srv__ExecMission_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ExecMission_Response>, size: usize) -> bool;
    fn mir_msgs__srv__ExecMission_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ExecMission_Response>);
    fn mir_msgs__srv__ExecMission_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ExecMission_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ExecMission_Response>) -> bool;
}

// Corresponds to mir_msgs__srv__ExecMission_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecMission_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ExecMission_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mir_msgs__srv__ExecMission_Response__init(&mut msg as *mut _) {
        panic!("Call to mir_msgs__srv__ExecMission_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ExecMission_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__srv__ExecMission_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__srv__ExecMission_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mir_msgs__srv__ExecMission_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ExecMission_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ExecMission_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mir_msgs/srv/ExecMission_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mir_msgs__srv__ExecMission_Response() }
  }
}






#[link(name = "mir_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mir_msgs__srv__ExecMission() -> *const std::ffi::c_void;
}

// Corresponds to mir_msgs__srv__ExecMission
#[allow(missing_docs, non_camel_case_types)]
pub struct ExecMission;

impl rosidl_runtime_rs::Service for ExecMission {
    type Request = ExecMission_Request;
    type Response = ExecMission_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mir_msgs__srv__ExecMission() }
    }
}


