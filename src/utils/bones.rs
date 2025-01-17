#[repr(usize)]
pub enum Bones {
    BoneHead = 6,
    BoneNeck = 5,
    BoneSpine = 4,
    BoneSpine1 = 2,
    BoneHip = 0,
    BoneLeftShoulder = 8,
    BoneLeftArm = 9,
    BoneLeftHand = 10,
    BoneRightShoulder = 13,
    BoneRightArm = 14,
    BoneRightHand = 15,

    BoneLeftHip = 22,
    BoneLeftKnee = 23,
    BoneLeftFeet = 24,

    BoneRightHip = 25,
    BoneRightKnee = 26,
    BoneRightFeet = 27,
}

pub struct BoneConnection {
    pub bone1: usize,
    pub bone2: usize,
}

impl BoneConnection {
    pub fn get() -> Vec<BoneConnection> {
        vec![
            BoneConnection {
                bone1: Bones::BoneHead as usize,
                bone2: Bones::BoneNeck as usize,
            },
            BoneConnection {
                bone1: Bones::BoneNeck as usize,
                bone2: Bones::BoneSpine as usize,
            },
            BoneConnection {
                bone1: Bones::BoneSpine as usize,
                bone2: Bones::BoneHip as usize,
            },
            BoneConnection {
                bone1: Bones::BoneSpine as usize,
                bone2: Bones::BoneLeftShoulder as usize,
            },
            BoneConnection {
                bone1: Bones::BoneLeftShoulder as usize,
                bone2: Bones::BoneLeftArm as usize,
            },
            BoneConnection {
                bone1: Bones::BoneLeftArm as usize,
                bone2: Bones::BoneLeftHand as usize,
            },
            BoneConnection {
                bone1: Bones::BoneSpine as usize,
                bone2: Bones::BoneRightShoulder as usize,
            },
            BoneConnection {
                bone1: Bones::BoneRightShoulder as usize,
                bone2: Bones::BoneRightArm as usize,
            },
            BoneConnection {
                bone1: Bones::BoneRightArm as usize,
                bone2: Bones::BoneRightHand as usize,
            },
            BoneConnection {
                bone1: Bones::BoneSpine as usize,
                bone2: Bones::BoneSpine1 as usize,
            },
            BoneConnection {
                bone1: Bones::BoneHip as usize,
                bone2: Bones::BoneLeftHip as usize,
            },
            BoneConnection {
                bone1: Bones::BoneHip as usize,
                bone2: Bones::BoneRightHip as usize,
            },
            BoneConnection {
                bone1: Bones::BoneLeftHip as usize,
                bone2: Bones::BoneLeftKnee as usize,
            },
            BoneConnection {
                bone1: Bones::BoneLeftKnee as usize,
                bone2: Bones::BoneLeftFeet as usize,
            },
            BoneConnection {
                bone1: Bones::BoneRightHip as usize,
                bone2: Bones::BoneRightKnee as usize,
            },
            BoneConnection {
                bone1: Bones::BoneRightKnee as usize,
                bone2: Bones::BoneRightFeet as usize,
            },
        ]
    }
}
