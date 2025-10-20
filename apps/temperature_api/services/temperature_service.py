from repositories.temperature_repository import TemperatureRepository
import datetime


class TemperatureService:
    """Сервисный слой для работы с температурными данными"""
    
    def __init__(self, repository: TemperatureRepository):
        self.repository = repository
    
    def get_temperature(self, location: str, sensor_id: str) -> dict:
        temperature = self.repository.get_random_temperature()

        # If no location is provided, use a default based on sensor ID
        if location is None or location == "":
            if sensor_id == "1":
                location = "Living Room"
            elif sensor_id == "2":
                location = "Bedroom"
            elif sensor_id == "3":
                location = "Kitchen"
            else:
                location = "Unknown"

        # If no sensor ID is provided, generate one based on location
        if sensor_id is None or sensor_id == "":
            if location == "Living Room":
                sensor_id = "1"
            elif location == "Bedroom":
                sensor_id = "2"
            elif location == "Kitchen":
                sensor_id = "3"
            else:    
                sensor_id = "0"

        return {
            "value": temperature,
            "unit": "celsius",
            "timestamp": datetime.datetime.now(),
            "location": location,
            "status": "test_data",
            "sensor_id": sensor_id,
            "sensor_type": "temperature",
            "description": "test_data"
        }
