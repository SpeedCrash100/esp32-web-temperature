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
        // Calculate Dew Point using the simplified formula
        // T_dew = T - ((100 - RH) / 5)
        const dewPoint = temperature - ((100 - humidity) / 5);
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