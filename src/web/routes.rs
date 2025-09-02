use picoserve::{
    extract::State,
    response::{DebugValue, IntoResponse, IntoResponseWithState},
};

use crate::web::{AppState, SharedTemp};

pub async fn get_temperature(
    State(state): State<SharedTemp>,
) -> impl IntoResponseWithState<AppState> {
    let celsius = state.get().await;
    DebugValue(celsius)
}
