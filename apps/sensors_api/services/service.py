"""Сервисный слой с валидацией данных"""

from datetime import datetime
from typing import List, Dict, Any
from repositories.repository import PostgresRepository
from models.schemas import SensorCreate, SensorUpdate, SensorResponse
from pydantic import ValidationError as PydanticValidationError
from exceptions import ValidationError


class SensorService:
    """Сервисный слой для работы с сенсорами"""
    
    def __init__(self, repository: PostgresRepository):
        self.repository = repository
    
    def get_sensors(self) -> List[Dict[str, Any]]:
        """Получить все сенсоры"""
        sensors = self.repository.get_sensors()
        # Валидация ответа из БД
        return [SensorResponse(**sensor).model_dump() for sensor in sensors]
    
    def create_sensor(self, data: dict) -> Dict[str, Any]:
        """
        Создать новый сенсор с валидацией
        
        Args:
            data: Сырые данные от клиента
            
        Returns:
            Словарь с созданным сенсором
            
        Raises:
            ValidationError: При невалидных данных
        """
        try:
            # Валидация входных данных
            sensor_data = SensorCreate(**data)
        except PydanticValidationError as e:
            raise ValidationError(f"Invalid input data: {str(e)}")
        
        # Добавляем временные метки
        # mode='python' возвращает enum объекты, которые наследуют str
        db_data = sensor_data.model_dump(mode='python')
        now = datetime.now()
        db_data['last_updated'] = now
        db_data['created_at'] = now
        
        # Enum наследуются от str, psycopg2 автоматически их обработает
        # Но для явности конвертируем в строки
        db_data['type'] = str(db_data['type'].value)
        
        created_sensor = self.repository.create_sensor(db_data)
        return SensorResponse(**created_sensor).model_dump()
    
    def get_sensor_by_id(self, sensor_id: int) -> Dict[str, Any]:
        """Получить сенсор по ID с валидацией"""
        if not isinstance(sensor_id, int) or sensor_id <= 0:
            raise ValidationError("Invalid sensor ID")
        
        sensor = self.repository.get_sensor_by_id(sensor_id)
        return SensorResponse(**sensor).model_dump()
    
    def update_sensor(self, sensor_id: int, data: dict) -> Dict[str, Any]:
        """
        Обновить сенсор с валидацией
        
        Args:
            sensor_id: ID сенсора
            data: Сырые данные от клиента
            
        Returns:
            Словарь с обновленным сенсором
        """
        if not isinstance(sensor_id, int) or sensor_id <= 0:
            raise ValidationError("Invalid sensor ID")
        
        try:
            # Валидация входных данных (все поля опциональны)
            sensor_data = SensorUpdate(**data)
        except PydanticValidationError as e:
            raise ValidationError(f"Invalid input data: {str(e)}")
        
        # Исключаем None значения
        db_data = {k: v for k, v in sensor_data.model_dump(mode='python').items() if v is not None}
        
        if not db_data:
            raise ValidationError("No fields to update")
        
        # Конвертируем enum в строки для БД
        if 'type' in db_data:
            db_data['type'] = str(db_data['type'].value)
        
        updated_sensor = self.repository.update_sensor(sensor_id, db_data)
        return SensorResponse(**updated_sensor).model_dump()
    
    def delete_sensor(self, sensor_id: int) -> bool:
        """Удалить сенсор"""
        if not isinstance(sensor_id, int) or sensor_id <= 0:
            raise ValidationError("Invalid sensor ID")
        
        return self.repository.delete_sensor(sensor_id)