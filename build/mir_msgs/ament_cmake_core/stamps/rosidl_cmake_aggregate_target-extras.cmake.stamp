# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target mir_msgs::mir_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${mir_msgs_TARGETS}.
if(mir_msgs_TARGETS AND NOT TARGET mir_msgs::mir_msgs)
  add_library(mir_msgs::mir_msgs INTERFACE IMPORTED)
  set_target_properties(mir_msgs::mir_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${mir_msgs_TARGETS}")
endif()
