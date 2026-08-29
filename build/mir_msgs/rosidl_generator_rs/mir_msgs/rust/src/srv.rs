#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to mir_msgs__srv__ExecMission_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecMission_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mission_name: std::string::String,

}



impl Default for ExecMission_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ExecMission_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ExecMission_Request {
  type RmwMsg = super::srv::rmw::ExecMission_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mission_name: msg.mission_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mission_name: msg.mission_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mission_name: msg.mission_name.to_string(),
    }
  }
}


// Corresponds to mir_msgs__srv__ExecMission_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExecMission_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ExecMission_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ExecMission_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ExecMission_Response {
  type RmwMsg = super::srv::rmw::ExecMission_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        message: msg.message.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        message: msg.message.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      message: msg.message.to_string(),
      success: msg.success,
    }
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


