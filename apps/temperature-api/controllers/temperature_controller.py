from flask import jsonify
from services.temperature_service import TemperatureService


class TemperatureController:
    """Контроллер для обработки HTTP запросов температуры"""
    
    def __init__(self, temperature_service: TemperatureService):
        self.temperature_service = temperature_service
    
    def get_test_temperature(self):
        """GET /test - возвращает случайное значение температуры"""
        try:
            result = self.temperature_service.get_test_temperature()
            return jsonify(result), 200
        except Exception as e:
            return jsonify({"error": str(e)}), 500
    
    def health_check(self):
        """GET /health - проверка состояния приложения"""
        return jsonify({"status": "healthy"}), 200
