"""Контроллер"""

from flask import jsonify, request, Request
from functools import wraps
from typing import Callable, Any, Tuple
from services.service import SensorService
from exceptions import (
    SensorAPIException,
    ValidationError,
    PayloadTooLargeError,
    UnsupportedMediaTypeError
)
import logging
logger = logging.getLogger(__name__)


# Лимиты
MAX_CONTENT_LENGTH = 1024 * 1024  # 1MB
MAX_JSON_PAYLOAD = 100 * 1024  # 100KB


def validate_content_type(f: Callable) -> Callable:
    """Декоратор для проверки Content-Type"""
    @wraps(f)
    def decorated_function(*args, **kwargs):
        if request.method in ['POST', 'PUT', 'PATCH']:
            content_type = request.headers.get('Content-Type', '')
            if not content_type.startswith('application/json'):
                logger.error(f"Unsupported Media Type: {content_type}")
                raise UnsupportedMediaTypeError()
        return f(*args, **kwargs)
    return decorated_function


def validate_payload_size(f: Callable) -> Callable:
    """Декоратор для проверки размера payload"""
    @wraps(f)
    def decorated_function(*args, **kwargs):
        if request.method in ['POST', 'PUT', 'PATCH']:
            content_length = request.content_length
            if content_length and content_length > MAX_JSON_PAYLOAD:
                logger.error(f"Payload too large: {content_length}")
                raise PayloadTooLargeError(
                    f"Payload size {content_length} exceeds limit {MAX_JSON_PAYLOAD}"
                )
            
            # Дополнительная проверка после парсинга
            if request.is_json:
                data = request.get_json()
                if data and len(str(data)) > MAX_JSON_PAYLOAD:
                    logger.error(f"JSON payload too large: {len(str(data))}")
                    raise PayloadTooLargeError("JSON payload too large")
        
        return f(*args, **kwargs)
    return decorated_function


def error_handler(f: Callable) -> Callable:
    """Централизованная обработка ошибок"""
    @wraps(f)
    def decorated_function(*args, **kwargs):
        try:
            return f(*args, **kwargs)
        except SensorAPIException as e:
            logger.error(f"Sensors API Exception: {str(e)} code: {e.status_code} message: {e.message}")
            return jsonify({
                "error": e.message,
                "status_code": e.status_code
            }), e.status_code
        except ValueError as e:
            logger.error(f"Invalid value: {str(e)}")
            return jsonify({
                "error": f"Invalid value: {str(e)}",
                "status_code": 400
            }), 400
        except Exception as e:
            # Не раскрываем внутренние ошибки клиенту
            logger.error(f"Error: {str(e)}")
            logger.exception(e)
            return jsonify({
                "error": "Internal server error",
                "status_code": 500
            }), 500
    return decorated_function


class SensorController:
    """Контроллер для обработки HTTP запросов сенсоров"""
    
    def __init__(self, sensor_service: SensorService):
        self.sensor_service = sensor_service
    
    @error_handler
    def get_sensors(self) -> Tuple[Any, int]:
        """Получить все сенсоры"""
        sensors = self.sensor_service.get_sensors()
        return jsonify(sensors), 200
    
    @error_handler
    @validate_content_type
    @validate_payload_size
    def create_sensor(self) -> Tuple[Any, int]:
        """Создать новый сенсор"""
        if not request.is_json:
            raise ValidationError("Request body must be JSON")
        
        data = request.get_json()
        if not data:
            raise ValidationError("Empty request body")
        
        sensor = self.sensor_service.create_sensor(data)
        return jsonify(sensor), 201
    
    @error_handler
    def get_sensor_by_id(self, id: int) -> Tuple[Any, int]:
        """Получить сенсор по ID"""
        sensor = self.sensor_service.get_sensor_by_id(id)
        return jsonify(sensor), 200
    
    @error_handler
    @validate_content_type
    @validate_payload_size
    def update_sensor(self, id: int) -> Tuple[Any, int]:
        """Обновить сенсор"""
        if not request.is_json:
            raise ValidationError("Request body must be JSON")
        
        data = request.get_json()
        if not data:
            raise ValidationError("Empty request body")
        
        sensor = self.sensor_service.update_sensor(id, data)
        return jsonify(sensor), 200
    
    @error_handler
    def delete_sensor(self, id: int) -> Tuple[Any, int]:
        """Удалить сенсор"""
        self.sensor_service.delete_sensor(id)
        return jsonify({"message": "Sensor deleted successfully"}), 200
    
    @error_handler
    def get_temperature_by_location(self, location: str) -> Tuple[Any, int]:
        """Получить данные датчика по локации"""
        sensor = self.sensor_service.get_sensor_by_location(location)
        if sensor is None:
            return jsonify({"error": "no sensor found", "status_code": 404}), 400
        return jsonify(sensor), 200
    
    @error_handler
    def health_check(self) -> Tuple[Any, int]:
        """Проверка здоровья сервиса"""
        return jsonify({"status": "healthy"}), 200