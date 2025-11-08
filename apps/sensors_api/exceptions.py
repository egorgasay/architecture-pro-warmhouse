"""Кастомные исключения для приложения"""


class SensorAPIException(Exception):
    """Базовый класс для всех исключений API"""
    def __init__(self, message: str, status_code: int = 500):
        self.message = message
        self.status_code = status_code
        super().__init__(self.message)


class ValidationError(SensorAPIException):
    """Ошибка валидации данных"""
    def __init__(self, message: str):
        super().__init__(message, status_code=400)


class DatabaseError(SensorAPIException):
    """Ошибка работы с БД"""
    def __init__(self, message: str):
        super().__init__(message, status_code=500)


class NotFoundError(SensorAPIException):
    """Ресурс не найден"""
    def __init__(self, message: str):
        super().__init__(message, status_code=404)


class PayloadTooLargeError(SensorAPIException):
    """Слишком большой payload"""
    def __init__(self, message: str = "Request payload too large"):
        super().__init__(message, status_code=413)


class UnsupportedMediaTypeError(SensorAPIException):
    """Неподдерживаемый Content-Type"""
    def __init__(self, message: str = "Content-Type must be application/json"):
        super().__init__(message, status_code=415)

