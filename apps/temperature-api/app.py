from flask import Flask
from controllers.temperature_controller import TemperatureController
from services.temperature_service import TemperatureService
from repositories.temperature_repository import RandomTemperatureRepository


def create_app():
    """Фабрика приложения с внедрением зависимостей"""
    app = Flask(__name__)
    
    # Создание экземпляров слоев
    repository = RandomTemperatureRepository()
    service = TemperatureService(repository)
    controller = TemperatureController(service)
    
    # Регистрация маршрутов
    app.add_url_rule('/test', 'test_temperature', controller.get_test_temperature, methods=['GET'])
    app.add_url_rule('/health', 'health_check', controller.health_check, methods=['GET'])
    
    return app


if __name__ == '__main__':
    app = create_app()
    app.run(host='0.0.0.0', port=5000, debug=True)
