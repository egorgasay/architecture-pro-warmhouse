from flask import Flask
from controllers.controller import SensorController
from services.service import SensorService
from repositories.repository import PostgresRepository
import os


def create_app():
    """Фабрика приложения с внедрением зависимостей"""
    app = Flask(__name__)
    
    # Создание экземпляров слоев
    repository = PostgresRepository(
        db_url=os.getenv("DATABASE_URL"),
    )
    service = SensorService(repository)
    controller = SensorController(service)
    
    # Регистрация маршрутов
    app.add_url_rule('/api/v1/sensors', 'get_sensors', controller.get_sensors, methods=['GET'])
    app.add_url_rule('/api/v1/sensors', 'create_sensor', controller.create_sensor, methods=['POST'])
    app.add_url_rule('/api/v1/sensors/<int:id>', 'get_sensor_by_id', controller.get_sensor_by_id, methods=['GET'])
    app.add_url_rule('/api/v1/sensors/<int:id>', 'update_sensor', controller.update_sensor, methods=['PUT'])
    app.add_url_rule('/api/v1/sensors/<int:id>', 'delete_sensor', controller.delete_sensor, methods=['DELETE'])
    app.add_url_rule('/health', 'health_check', controller.health_check, methods=['GET'])
    
    return app


if __name__ == '__main__':
    app = create_app()
    app.run(host='0.0.0.0', port=5000, debug=True)