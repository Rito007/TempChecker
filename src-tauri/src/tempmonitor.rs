use tauri::{AppHandle, Emitter};
use wmi::{COMLibrary, WMIConnection};
use std::{thread, time::Duration};
use tauri_plugin_notification::NotificationExt;
use serde::Deserialize;
use crate::{sound::play_notification_sound, AppState};

#[derive(Deserialize, Debug)]
struct ThermalZone {
    instance_name: Option<String>,
    current_temperature: i32,
    active: bool,
}

pub fn start_temperature_monitoring(app: AppHandle, state: AppState) {
    thread::spawn(move || {
        let com_con = COMLibrary::new().unwrap();
        let wmi_con = WMIConnection::with_namespace_path("ROOT\\WMI", com_con).unwrap();

        loop {
            let results: Vec<ThermalZone> = wmi_con
                .raw_query("SELECT InstanceName, CurrentTemperature, Active FROM MSAcpi_ThermalZoneTemperature")
                .unwrap_or_default();
            println!("{:?}", results);
            for temp in results {
                if temp.active && temp.current_temperature != 2732 {
                    let celsius = (temp.current_temperature as f64 - 2732.0) / 10.0;
                    let name = temp.instance_name.clone().unwrap_or_else(|| "Desconhecido".into());

                    println!("🌡️ {}: {:.1}°C", name, celsius);

                    {
                        let mut temperature = state.temperature.lock().unwrap();
                        *temperature = celsius as i16;
                    }

                    let limite = {
                        let guard = state.temperature_limit.lock();
                        match guard {
                            Ok(l) => *l,
                            Err(_) => 40,
                        }
                    };

                    if celsius as i16 > limite {
                        play_notification_sound();

                        let _ = app.notification()
                            .builder()
                            .title("🌡️ Temperatura Alta")
                            .body(&format!("CPU a {:.1}°C", celsius))
                            .show();
                    }
                }
            }

            thread::sleep(Duration::from_secs(10));
        }
    });
}