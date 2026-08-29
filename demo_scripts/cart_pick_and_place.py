#!/usr/bin/env python3
"""Mobile pick-and-place: the MiR cart drives to an object, the UR arm picks
it up, the cart drives to a different location, and the arm places the
object there.

Reuses the same scripted-attach trick as pick_and_place.py (Gazebo classic's
grasp plugin isn't reliable here): once the gripper closes, the object's
pose is continuously driven to follow the gripper via
/gazebo/set_entity_state -- including while the base is driving, so the
object visibly rides along with the cart.

Base motion uses ground-truth pose from /gazebo/model_states (not odometry,
which would drift) with a simple rotate-then-drive controller publishing
Twist on /diff_cont/cmd_vel_unstamped. There's no obstacle avoidance -- the
pickup point and the path to the drop-off were chosen to give the cart a
straight, clear route around the world's rocks/trees. The drop-off itself
is deliberately placed next to rock_2, an existing landmark in the world,
rather than an arbitrary empty spot.

Run against an already-running mir_gazebo_launch.py simulation:
    source /opt/ros/humble/setup.bash
    source /home/rahul/Mir_robot/install/setup.bash
    python3 demo_scripts/cart_pick_and_place.py
"""
import math
import time
import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy
from trajectory_msgs.msg import JointTrajectory, JointTrajectoryPoint
from std_msgs.msg import Float64MultiArray
from geometry_msgs.msg import Twist
from tf2_ros import Buffer, TransformListener
from gazebo_msgs.srv import SetEntityState
from gazebo_msgs.msg import EntityState, ModelStates

JOINTS = ['ur_shoulder_pan_joint', 'ur_shoulder_lift_joint', 'ur_elbow_joint',
          'ur_wrist_1_joint', 'ur_wrist_2_joint', 'ur_wrist_3_joint']

HOME = [0.0, -1.5708, 0.0, 0.0, 1.5708, 0.0]
REACH = [0.0, 0.45, 0.95, 0.0, 1.5708, 0.0]
LIFT = [0.0, -0.3, 0.6, 0.0, 1.5708, 0.0]

GRIP_OPEN = 0.0
GRIP_CLOSED = 0.65
GRASP_Z_OFFSET = -0.10

# Object sits this far along +y in base_footprint frame when graspable
# (matches the REACH pose tuning from pick_and_place.py / try_pose.sh).
# Both pickup AND place reuse this exact same forward-facing REACH pose
# (shoulder_pan stays 0 throughout the whole script) instead of swinging
# the arm out to shoulder_pan=0.8 for placing -- that side reach visually
# clipped through the MiR chassis since it brings the gripper back in over
# the robot's own footprint. Forward reach never has that problem.
OBJECT_LOCAL_OFFSET = (0.0, 0.39)

HOME_BASE = (0.0, 0.0, 0.0)
OBJECT_WORLD_POS = (2.0, 1.5, 0.09)

# rock_2 in outdoor_small.world sits at (-1.5, -1.0) -- drop the object
# ~0.6m clear of it (rock's half-extent is ~0.15m, MiR's footprint half
# width is ~0.3-0.45m) so it visibly lands next to that landmark without
# the cart driving into it.
ROCK2_POS = (-1.5, -1.0)
DROPOFF_OBJECT_POS = (ROCK2_POS[0] + 0.6, ROCK2_POS[1])

PICKUP_BASE = (OBJECT_WORLD_POS[0] - OBJECT_LOCAL_OFFSET[0],
               OBJECT_WORLD_POS[1] - OBJECT_LOCAL_OFFSET[1],
               0.0)
DROPOFF_BASE = (DROPOFF_OBJECT_POS[0] - OBJECT_LOCAL_OFFSET[0],
                DROPOFF_OBJECT_POS[1] - OBJECT_LOCAL_OFFSET[1],
                0.0)


def normalize_angle(a):
    while a > math.pi:
        a -= 2 * math.pi
    while a < -math.pi:
        a += 2 * math.pi
    return a


class CartPickAndPlace(Node):
    def __init__(self):
        super().__init__('cart_pick_and_place')
        reliable_qos = QoSProfile(depth=10, reliability=ReliabilityPolicy.RELIABLE,
                                   history=HistoryPolicy.KEEP_LAST)
        self.traj_pub = self.create_publisher(JointTrajectory,
                                               '/joint_trajectory_controller/joint_trajectory',
                                               reliable_qos)
        self.grip_pub = self.create_publisher(Float64MultiArray,
                                               '/gripper_controller/commands',
                                               reliable_qos)
        self.cmd_vel_pub = self.create_publisher(Twist,
                                                  '/diff_cont/cmd_vel_unstamped',
                                                  reliable_qos)
        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)
        self.set_state_cli = self.create_client(SetEntityState, '/gazebo/set_entity_state')

        self.base_pose = None  # (x, y, yaw), ground truth from Gazebo
        self.create_subscription(ModelStates, '/gazebo/model_states', self._model_states_cb, 10)

        self._wait_for_subs(self.traj_pub)
        self._wait_for_subs(self.grip_pub)
        self._wait_for_subs(self.cmd_vel_pub)
        self.set_state_cli.wait_for_service(timeout_sec=10.0)
        self._wait_for_base_pose()

    def _wait_for_subs(self, pub, timeout=10.0):
        t0 = time.time()
        while pub.get_subscription_count() < 1 and time.time() - t0 < timeout:
            rclpy.spin_once(self, timeout_sec=0.1)

    def _wait_for_base_pose(self, timeout=10.0):
        t0 = time.time()
        while self.base_pose is None and time.time() - t0 < timeout:
            rclpy.spin_once(self, timeout_sec=0.1)

    def _model_states_cb(self, msg):
        try:
            i = msg.name.index('mir_robot')
        except ValueError:
            return
        p = msg.pose[i].position
        o = msg.pose[i].orientation
        yaw = math.atan2(2 * (o.w * o.z + o.x * o.y), 1 - 2 * (o.y * o.y + o.z * o.z))
        self.base_pose = (p.x, p.y, yaw)

    def spin_sleep(self, duration, track=False):
        t0 = time.time()
        while time.time() - t0 < duration:
            rclpy.spin_once(self, timeout_sec=0.05)
            if track:
                self.track_object_to_gripper()

    def send_arm(self, positions, duration_sec=2.5, settle=1.0, track=False):
        traj = JointTrajectory()
        traj.joint_names = JOINTS
        pt = JointTrajectoryPoint()
        pt.positions = positions
        pt.time_from_start.sec = int(duration_sec)
        pt.time_from_start.nanosec = int((duration_sec % 1) * 1e9)
        traj.points = [pt]
        for _ in range(3):
            self.traj_pub.publish(traj)
            self.spin_sleep(0.1, track=track)
        self.spin_sleep(duration_sec + settle, track=track)

    def send_gripper(self, value, settle=1.0):
        msg = Float64MultiArray()
        msg.data = [value]
        for _ in range(3):
            self.grip_pub.publish(msg)
            self.spin_sleep(0.1)
        self.spin_sleep(settle)

    def gripper_pose_local(self):
        tf = self.tf_buffer.lookup_transform('base_footprint', 'robotiq_85_base_link',
                                              rclpy.time.Time())
        return tf.transform.translation

    def set_object_pose(self, x, y, z, qx=0.0, qy=0.0, qz=0.0, qw=1.0, blocking=True):
        req = SetEntityState.Request()
        req.state = EntityState()
        req.state.name = 'pickup_object'
        req.state.pose.position.x = x
        req.state.pose.position.y = y
        req.state.pose.position.z = z
        req.state.pose.orientation.x = qx
        req.state.pose.orientation.y = qy
        req.state.pose.orientation.z = qz
        req.state.pose.orientation.w = qw
        req.state.reference_frame = 'world'
        req.state.twist.linear.x = 0.0
        req.state.twist.linear.y = 0.0
        req.state.twist.linear.z = 0.0
        req.state.twist.angular.x = 0.0
        req.state.twist.angular.y = 0.0
        req.state.twist.angular.z = 0.0
        future = self.set_state_cli.call_async(req)
        if blocking:
            rclpy.spin_until_future_complete(self, future, timeout_sec=1.0)
        # else: fire-and-forget. Waiting here would block whatever loop is
        # calling us (e.g. the base-drive control loop), starving cmd_vel
        # publishing until diff_drive_controller's cmd_vel_timeout kicks in
        # and stalls the base. The next tick sends a fresher pose anyway.

    def set_base_pose(self, x, y, yaw):
        req = SetEntityState.Request()
        req.state = EntityState()
        req.state.name = 'mir_robot'
        req.state.pose.position.x = x
        req.state.pose.position.y = y
        req.state.pose.position.z = 0.0
        req.state.pose.orientation.z = math.sin(yaw / 2.0)
        req.state.pose.orientation.w = math.cos(yaw / 2.0)
        req.state.reference_frame = 'world'
        future = self.set_state_cli.call_async(req)
        rclpy.spin_until_future_complete(self, future, timeout_sec=1.0)

    _last_track_time = 0.0

    def track_object_to_gripper(self):
        # gripper_pose_local() is base_footprint-relative (unaffected by the
        # base driving around); compose it with the base's own world pose so
        # the object lands at the gripper's true WORLD position, not just its
        # offset from wherever base_footprint happens to be right now.
        #
        # Throttled to ~8Hz: calling /gazebo/set_entity_state every single
        # control tick (~20Hz+) noticeably drags down Gazebo's real-time
        # factor under this machine's load (confirmed empirically -- with
        # per-tick tracking a drive_to() that converges in ~13s with no
        # tracking took ~55s and often didn't converge at all). 8Hz is still
        # visually seamless for an object riding along with the gripper.
        now = time.time()
        if now - self._last_track_time < 0.12:
            return
        self._last_track_time = now
        if self.base_pose is None:
            return
        try:
            t = self.gripper_pose_local()
        except Exception:
            return  # TF can briefly glitch mid-drive; skip this tick
        bx, by, byaw = self.base_pose
        lx, ly, lz = t.x, t.y, t.z + GRASP_Z_OFFSET
        wx = bx + lx * math.cos(byaw) - ly * math.sin(byaw)
        wy = by + lx * math.sin(byaw) + ly * math.cos(byaw)
        self.set_object_pose(wx, wy, lz, blocking=False)

    def stop_base(self):
        self.cmd_vel_pub.publish(Twist())

    def drive_to(self, target_x, target_y, target_yaw, track=False,
                 pos_tol=0.05, yaw_tol=0.05, max_lin=0.35, max_ang=0.6, timeout=40.0):
        t0 = time.time()
        # Phase 1: rotate to face the target, then drive to it (with mild
        # continuous heading correction).
        while time.time() - t0 < timeout:
            rclpy.spin_once(self, timeout_sec=0.05)
            if track:
                self.track_object_to_gripper()
            x, y, yaw = self.base_pose
            dx, dy = target_x - x, target_y - y
            dist = math.hypot(dx, dy)
            if dist < pos_tol:
                break
            heading_err = normalize_angle(math.atan2(dy, dx) - yaw)
            cmd = Twist()
            if abs(heading_err) > 0.2:
                cmd.angular.z = max(-max_ang, min(max_ang, 2.0 * heading_err))
            else:
                # Floor the speed so approach doesn't asymptotically crawl
                # as dist shrinks -- plain P-control on distance alone can
                # take forever to cross the last few centimeters.
                cmd.linear.x = max(0.08, min(max_lin, 0.6 * dist))
                cmd.angular.z = max(-max_ang, min(max_ang, 1.5 * heading_err))
            self.cmd_vel_pub.publish(cmd)
        self.stop_base()

        # Phase 2: rotate in place to the final desired heading. Fresh
        # timeout budget -- a slow phase 1 shouldn't starve this phase.
        t0 = time.time()
        while time.time() - t0 < timeout:
            rclpy.spin_once(self, timeout_sec=0.05)
            if track:
                self.track_object_to_gripper()
            _, _, yaw = self.base_pose
            yaw_err = normalize_angle(target_yaw - yaw)
            if abs(yaw_err) < yaw_tol:
                break
            cmd = Twist()
            cmd.angular.z = max(-max_ang, min(max_ang, 2.0 * yaw_err))
            self.cmd_vel_pub.publish(cmd)
        self.stop_base()
        self.spin_sleep(0.3, track=track)


def main():
    rclpy.init()
    node = CartPickAndPlace()

    print("0) resetting: base -> home, object -> pickup spot, arm -> home")
    node.set_base_pose(*HOME_BASE)
    node.set_object_pose(*OBJECT_WORLD_POS)
    node.send_gripper(GRIP_OPEN, settle=0.3)
    node.send_arm(HOME, duration_sec=2.5, settle=0.5)
    node.spin_sleep(1.0)

    print(f"1) driving cart to pickup location {PICKUP_BASE[:2]}")
    node.drive_to(*PICKUP_BASE)

    print("2) reaching down and grasping the object")
    node.send_arm(REACH, duration_sec=3.0, settle=1.0)
    node.track_object_to_gripper()
    node.send_gripper(GRIP_CLOSED, settle=1.0)
    node.track_object_to_gripper()

    print("3) lifting object clear of the ground")
    node.send_arm(LIFT, duration_sec=2.0, settle=0.5, track=True)

    print(f"4) driving cart to drop-off location {DROPOFF_BASE[:2]}, carrying object")
    node.drive_to(*DROPOFF_BASE, track=True)

    print("5) lowering and placing the object (same forward reach as pickup --")
    print("   no side-swing, so the arm never sweeps back over the chassis)")
    node.send_arm(REACH, duration_sec=2.5, settle=0.5, track=True)
    node.send_gripper(GRIP_OPEN, settle=1.0)

    print("6) retreating straight up, then arm home")
    node.send_arm(LIFT, duration_sec=2.0, settle=0.5)
    node.send_arm(HOME, duration_sec=2.5, settle=1.0)
    # Deliberately not driving back to HOME_BASE here: the object is placed
    # close enough to the chassis (same ~0.39m forward reach as pickup) that
    # an in-place turn/drive-off would clip and drag it (confirmed by
    # testing in the earlier version of this script). Leave the cart parked
    # next to the object it just placed.

    print("done")
    rclpy.shutdown()


if __name__ == '__main__':
    main()
