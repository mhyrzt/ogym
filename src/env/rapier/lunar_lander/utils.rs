use rapier2d::prelude::{ImpulseJointHandle, RigidBodyHandle};

pub const CHUNKS: usize = 11;
pub const MIDDLE: usize = CHUNKS / 2;
pub const LANDER_POLY: [(i32, i32); 6] = [
    (-14, 17),
    (-17, 0),
    (-17, -10),
    (17, -10),
    (17, 0),
    (14, 17),
];
pub const LANDER_POLY_WIDTH: f32 = 34.0;

#[derive(Debug, Clone, Copy, Default)]
pub struct Helipad {
    pub y: f32,
    pub x1: f32,
    pub x2: f32,
}

pub struct Leg {
    pub body: RigidBodyHandle,
    pub joint: ImpulseJointHandle,
    pub ground_contact: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rapier2d::prelude::{ImpulseJointHandle, RigidBodyHandle};

    #[test]
    fn test_constants_validity() {
        assert_eq!(CHUNKS, 11);
        assert_eq!(MIDDLE, 5); // 11 / 2 integer division
        assert_eq!(LANDER_POLY_WIDTH, 34.0);
        assert_eq!(LANDER_POLY.len(), 6);
    }

    #[test]
    fn test_lander_poly_values() {
        let expected = [
            (-14, 17),
            (-17, 0),
            (-17, -10),
            (17, -10),
            (17, 0),
            (14, 17),
        ];
        assert_eq!(LANDER_POLY, expected);
    }

    #[test]
    fn test_helipad_default() {
        let helipad = Helipad::default();
        assert_eq!(helipad.y, 0.0);
        assert_eq!(helipad.x1, 0.0);
        assert_eq!(helipad.x2, 0.0);
    }

    #[test]
    fn test_helipad_instantiation() {
        let helipad = Helipad {
            y: 10.5,
            x1: -5.0,
            x2: 5.0,
        };
        assert_eq!(helipad.y, 10.5);
        assert_eq!(helipad.x1, -5.0);
        assert_eq!(helipad.x2, 5.0);
    }

    #[test]
    fn test_leg_instantiation() {
        let body_handle = RigidBodyHandle::from_raw_parts(1, 0);
        let joint_handle = ImpulseJointHandle::from_raw_parts(2, 0);

        let leg = Leg {
            body: body_handle,
            joint: joint_handle,
            ground_contact: true,
        };

        assert_eq!(leg.body, body_handle);
        assert_eq!(leg.joint, joint_handle);
        assert!(leg.ground_contact);
    }
}
