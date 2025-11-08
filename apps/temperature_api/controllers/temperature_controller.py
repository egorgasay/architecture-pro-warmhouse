from flask import jsonify
from flask import request
from services.temperature_service import TemperatureService


class TemperatureController:
    """Контроллер для обработки HTTP запросов температуры"""
    
    def __init__(self, temperature_service: TemperatureService):
        self.temperature_service = temperature_service
    
    def get_temperature(self):
        try:
            location = request.args.get("location", None)
            sensor_id = request.args.get("sensor_id", None)
            result = self.temperature_service.get_temperature(location, sensor_id)
            return jsonify(result), 200
        except Exception as e:
            return jsonify({"error": str(e)}), 500
    
    def health_check(self):
        """GET /health - проверка состояния приложения"""
        return jsonify({"status": "healthy"}), 200
