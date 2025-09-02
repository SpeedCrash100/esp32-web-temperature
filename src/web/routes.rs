use picoserve::{
    extract::State,
    response::{DebugValue, IntoResponseWithState},
};

use crate::web::{AppState, SharedHumidity, SharedTemp};

pub async fn get_temperature(
    State(state): State<SharedTemp>,
) -> impl IntoResponseWithState<AppState> {
    let celsius = state.get().await;
    DebugValue(celsius)
}

pub async fn get_humidity(
    State(state): State<SharedHumidity>,
) -> impl IntoResponseWithState<AppState> {
    let percentage = state.get().await;
    DebugValue(percentage)
}
