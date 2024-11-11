use directories::ProjectDirs;
use localsend_lib_types::messages::common_fields::{DeviceInfo, DeviceType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct State {
    pub(crate) device_info: DeviceInfo,
}

impl Default for State {
    fn default() -> Self {
        // Generate
        let alias = names::Generator::default().next().expect("Infinite names");
        let fingerprint = uuid::Uuid::new_v4().to_string();
        // Detect
        let device_model = std::env::consts::OS.to_string();
        // Fixed
        let device_type = DeviceType::Headless;
        // Combine
        let device_info = DeviceInfo::new(
            alias.into(),
            Some(device_model.into()),
            device_type,
            fingerprint.into(),
        );
        State { device_info }
    }
}

pub(crate) fn load_state() -> State {
    if let Some(project_dirs) = ProjectDirs::from("", "Nainapps", "localsend.rs") {
        let generated_state_dir = project_dirs.data_local_dir();
        let generated_state_file = generated_state_dir.join("state.toml");
        if generated_state_file.exists() {
            if let Ok(state_string) = std::fs::read_to_string(generated_state_file) {
                let state: State = toml::from_str(&state_string).expect("state altered?");
                return state;
            }
        } else {
            let state = State::default();
            let _ = std::fs::create_dir_all(generated_state_dir);
            let _ = std::fs::write(
                generated_state_file,
                toml::to_string_pretty(&state).unwrap(),
            );
            return state;
        }
    };
    State::default()
}
