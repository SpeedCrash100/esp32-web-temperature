async function updateReadings() {
    let temperature = null;
    let humidity = null;

    // Fetch Temperature
    try {
        const temperatureResponse = await fetch('/temperature');
        temperature = parseFloat(await temperatureResponse.text()); // Parse as float
        document.getElementById('temperature-value').textContent = temperature.toFixed(1); // Display with 1 decimal
    } catch (error) {
        console.error('Error fetching temperature:', error);
        document.getElementById('temperature-value').textContent = 'Error';
    }

    // Fetch Humidity
    try {
        const humidityResponse = await fetch('/humidity');
        humidity = parseFloat(await humidityResponse.text()); // Parse as float
        document.getElementById('humidity-value').textContent = humidity.toFixed(1); // Display with 1 decimal
    } catch (error) {
        console.error('Error fetching humidity:', error);
        document.getElementById('humidity-value').textContent = 'Error';
    }

    // Calculate and Display Dew Point if both values were fetched successfully
    if (temperature !== null && humidity !== null && !isNaN(temperature) && !isNaN(humidity)) {
        // Calculate Dew Point using an approximation of the Magnus-Tetens equation for better accuracy
        // Formula: T_dew = T - ((100 - RH) / 5) is a simplification.
        // A more accurate formula is based on the Clausius-Clapeyron relation.
        // One common approximation is:
        // T_dew = T - ((100 - %RH) / 5) <-- Original simplified formula
        // A more complex but accurate version involves the saturation vapor pressure.
        // Let's use an established approximation:
        // e_s(T) = 0.6108 * exp((17.27 * T) / (T + 237.3))  [Saturation vapor pressure in kPa]
        // e(T, RH) = e_s(T) * (RH / 100)                  [Actual vapor pressure]
        // T_dew = (237.3 * ln(e(T, RH) / 0.6108)) / (17.27 - ln(e(T, RH) / 0.6108)) [Dew point temperature in °C]

        // For practical purposes in JavaScript, we can use a simplified but more accurate formula
        // derived from the above:
        const a = 17.27;
        const b = 237.7;
        const gamma = ((humidity / 100) * 6.112 * Math.exp((a * temperature) / (b + temperature))) / 6.112;
        const dewPoint = (b * Math.log(gamma)) / (a - Math.log(gamma));
        document.getElementById('dewpoint-value').textContent = dewPoint.toFixed(1); // Display with 1 decimal
    } else {
        // If either value failed or is invalid, show N/A or Error for dew point
        if (document.getElementById('dewpoint-value')) { // Check if element exists before trying to update
            document.getElementById('dewpoint-value').textContent = 'N/A';
        }
    }
}

// Update readings every 5 seconds
setInterval(updateReadings, 5000);

// Initial update when the page loads
document.addEventListener('DOMContentLoaded', updateReadings);