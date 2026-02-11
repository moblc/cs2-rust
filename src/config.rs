#![allow(non_snake_case, non_upper_case_globals)]

pub mod Offsets {
    // https://raw.githubusercontent.com/a2x/cs2-dumper/refs/heads/main/output/offsets.rs
    pub mod client_dll {
        pub const dwEntityList: usize = 0x24AA0D8;
        pub const dwLocalPlayerController: usize = 0x22EF0B8;
        pub const dwLocalPlayerPawn: usize = 0x2064AE0;
        pub const dwPlantedC4: usize = 0x23120B0;
        pub const dwViewAngles: usize = 0x2314F98;
        pub const dwViewMatrix: usize = 0x230ADE0;
    }

    // https://raw.githubusercontent.com/a2x/cs2-dumper/refs/heads/main/output/client_dll.rs
    pub mod C_BaseEntity {
        pub const m_iHealth: usize = 0x354; // int32
        pub const m_iTeamNum: usize = 0x3F3; // uint8
        pub const m_pGameSceneNode: usize = 0x338; // CGameSceneNode*
        pub const m_fFlags: usize = 0x400; // uint32
        pub const m_nSubclassID: usize = 0x388; // CUtlStringToken
    }

    pub mod CBasePlayerController {
        pub const m_hPawn: usize = 0x6C4; // CHandle<C_BasePlayerPawn>
        pub const m_iszPlayerName: usize = 0x6F8; // char[128]
    }

    pub mod CCSPlayerController {
        pub const m_hPlayerPawn: usize = 0x90C; // CHandle<C_CSPlayerPawn>
        pub const m_bPawnIsAlive: usize = 0x914; // bool
    }

    pub mod C_BasePlayerPawn {
        pub const m_pObserverServices: usize = 0x13F0; // CPlayer_ObserverServices*
        pub const m_pCameraServices: usize = 0x1410; // CPlayer_CameraServices*
        pub const m_vOldOrigin: usize = 0x1588; // Vector
    }

    pub mod C_CSPlayerPawnBase {
        pub const m_vecLastClipCameraPos: usize = 0x3DA4; // Vector
        pub const m_angEyeAngles: usize = 0x3DD0; // QAngle
        pub const m_pClippingWeapon: usize = 0x3DC0; // C_CSWeaponBase*
        pub const m_iIDEntIndex: usize = 0x3EAC; // CEntityIndex
        pub const m_entitySpottedState: usize = 0x26E0; // EntitySpottedState_t
        pub const m_ArmorValue: usize = 0x272C; // int32
        pub const m_iShotsFired: usize = 0x270C; // int32
    }

    pub mod C_CSPlayerPawn {
        pub const m_aimPunchCache: usize = 0x16F0; // CUtlVector<QAngle>
    }

    pub mod CGameSceneNode {
        pub const m_vecAbsOrigin: usize = 0xD0; // Vector
    }

    pub mod CCSPlayerBase_CameraServices {
        pub const m_iFOVStart: usize = 0x294; // uint32
    }

    pub mod EntitySpottedState_t {
        pub const m_bSpottedByMask: usize = 0xC; // uint32[2]
    }

    pub mod CSkeletonInstance {
        pub const m_modelState: usize = 0x160; // CModelState
    }

    pub mod CPlayer_ObserverServices {
        pub const m_hObserverTarget: usize = 0x4C; // CHandle<C_BaseEntity>
    }

    pub mod C_PlantedC4 {
        pub const m_nBombSite: usize = 0x1174; // int32
    }

    pub mod CBasePlayerWeaponVData {
        pub const m_iMaxClip1: usize = 0x3F0; // int32
    }

    pub mod C_BasePlayerWeapon {
        pub const m_iClip1: usize = 0x18D0; // int32
    }
}

pub mod ProgramConfig {
    pub mod Package {
        pub const Name: &str = "cs2-helper";
        pub const Description: &str = "An open-source, external CS2 cheat.";
        pub const Executable: &str = "cs2-helper.exe";
        pub const Version: &str = env!("CARGO_PKG_VERSION");
        pub const Authors: &str = &env!("CARGO_PKG_AUTHORS");
    }

    pub mod Imgui {
        pub const FontSize: f32 = 13.0;

        pub mod FontPaths {
            pub const Chinese: &str = "C:/Windows/Fonts/msyh.ttc";
            pub const Cryillic: &str = "C:/Windows/Fonts/Arial.ttf";
            pub const Arabic: &str = "C:/Windows/Fonts/calibri.ttf";
        }
    }

    pub mod Update {
        pub const Enabled: bool = true;
        pub const URL: &str = "https://code.snipcola.st/snipcola/cs2-helper/raw/branch/main/bin/cs2-helper.exe";
        pub const CargoTomlURL: &str = "https://code.snipcola.st/snipcola/cs2-helper/raw/branch/main/Cargo.toml";
    }

    pub mod Links {
        pub const Source: &str = "https://code.snipcola.st/snipcola/cs2-helper";
        pub const License: &str = "https://code.snipcola.st/snipcola/cs2-helper/raw/branch/main/LICENSE";
    }

    pub mod Keys {
        use glutin::event::VirtualKeyCode;
        use mki::Keyboard;

        pub const Available: [&str; 20] = ["Alt", "Left Mouse", "Middle Mouse", "Right Mouse", "Side Mouse", "Extra Mouse", "Shift", "Control", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"];

        pub const ToggleKey: VirtualKeyCode = VirtualKeyCode::Delete;
        pub const ToggleKeyMKI: Keyboard = Keyboard::Delete;

        pub const ExitKey: VirtualKeyCode = VirtualKeyCode::End;
        pub const ExitKeyMKI: Keyboard = Keyboard::Other(0x23);
    }

    pub mod TargetProcess {
        pub const Executable: &str = "cs2.exe";
        pub const MaxAttempts: u32 = 30;
        pub const InitAddressesMaxAttempts: u32 = 15;

        pub mod Window {
            pub const Title: &str = "Counter-Strike 2";
            pub const Class: &str = "SDL_app";
        }
    }

    pub mod CheckDelays {
        use std::time::Duration;

        pub const AttachProcess: Duration = Duration::from_millis(1000);
        pub const InitAddresses: Duration = Duration::from_millis(1000);
    }

    pub mod ThreadDelays {
        use std::time::Duration;

        pub const UpdateConfigs: Duration = Duration::from_millis(250);
        pub const WindowTasks: Duration = Duration::from_millis(25);
        pub const IOTasks: Duration = Duration::from_millis(25);
    }

    pub mod CheatDelays {
        use std::time::Duration;

        pub const Toggle: Duration = Duration::from_millis(200);
        pub const Aimbot: Duration = Duration::from_millis(10);
        pub const AimbotOffEntity: Duration = Duration::from_millis(500);
        pub const TriggerbotOffEntity: Duration = Duration::from_millis(500);
    }
}
