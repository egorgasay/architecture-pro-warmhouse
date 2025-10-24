from flask import Flask
from controllers.controller import SensorController
from services.service import SensorService
from repositories.repository import PostgresRepository
import os
import json
from datetime import datetime
from senders.statemon_client import StateMonitoringClient

def create_app():
    """Фабрика приложения с внедрением зависимостей"""
    app = Flask(__name__)
    
    
    # Создание экземпляров слоев
    repository = PostgresRepository(
        db_url=os.getenv("DATABASE_URL","postgresql://postgres:qwerty@127.0.0.1:6002/sensors_api"),
    )
    statemon_client = StateMonitoringClient(base_url=os.getenv("STATE_MONITORING_API_URL","http://127.0.0.1:7676"))
    service = SensorService(repository, statemon_client)
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
    app.run(host='0.0.0.0', port=8081, debug=True)