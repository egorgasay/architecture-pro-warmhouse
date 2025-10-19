from repositories.temperature_repository import TemperatureRepository


class TemperatureService:
    """Сервисный слой для работы с температурными данными"""
    
    def __init__(self, repository: TemperatureRepository):
        self.repository = repository
    
    def get_test_temperature(self) -> dict:
        temperature = self.repository.get_random_temperature()
        return {
            "temperature": temperature,
            "unit": "celsius",
            "status": "test_data"
        }
