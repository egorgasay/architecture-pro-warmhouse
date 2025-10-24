"""Сервисный слой с валидацией данных"""

from datetime import datetime
from typing import List, Dict, Any
from repositories.repository import PostgresRepository
from models.schemas import SensorCreate, SensorUpdate, SensorResponse
from pydantic import ValidationError as PydanticValidationError
from exceptions import ValidationError
import logging
from senders.statemon_client import StateMonitoringClient

logger = logging.getLogger(__name__)


class SensorService:
    """Сервисный слой для работы с сенсорами"""
    
    def __init__(self, repository: PostgresRepository, statemon_client: StateMonitoringClient):
        self.repository = repository
        self.statemon_client = statemon_client
    
    def get_data_from_statemon(self, sensor_id: int) -> Dict[str, Any]:
        """Получить данные из state_monitoring_api"""
        try:
            data = self.statemon_client.get_data(sensor_id)
            logger.info(f"Data from state_monitoring_api: {data}")
            return data
        except Exception as e:
            logger.error(f"Error getting data from state_monitoring_api for sensor {sensor_id}: {e}")
            return {}

    def get_sensors(self) -> List[Dict[str, Any]]:
        """Получить все сенсоры"""
        sensors = self.repository.get_sensors()
        if not sensors:
            return []
        
        result = []
        for sensor in sensors:
            data = self.get_data_from_statemon(sensor['id'])
            
            try:
                sensor_response = SensorResponse(
                    **sensor,
                    value=data.get('value', None),
                    unit=data.get('unit', None),
                    status=data.get('status', None),
                )
                result.append(sensor_response.model_dump())
            except Exception as e:
                logger.error(f"Error creating SensorResponse for sensor {sensor.get('id')}: {e}")
                continue
        return result
    
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
        
        db_data = sensor_data.model_dump(mode='python')
        now = datetime.now()
        db_data['last_updated'] = now
        db_data['created_at'] = now
        db_data['type'] = str(db_data['type'].value)
        
        created_sensor = self.repository.create_sensor(db_data)
        
        data = self.get_data_from_statemon(created_sensor['id'])
        sensor = SensorResponse(
            **created_sensor, 
            value=data.get('value', None), 
            unit=data.get('unit', None), 
            status=data.get('status', None)
        )
        return sensor.model_dump()
    
    def get_sensor_by_id(self, sensor_id: int) -> Dict[str, Any]:
        """Получить сенсор по ID с валидацией"""
        if not isinstance(sensor_id, int) or sensor_id <= 0:
            raise ValidationError("Invalid sensor ID")
        
        sensor = self.repository.get_sensor_by_id(sensor_id)
        data = self.get_data_from_statemon(sensor_id)

        sensor = SensorResponse(
            **sensor, 
            value=data.get('value', None), 
            unit=data.get('unit', None), 
            status=data.get('status', None)
        )
        return sensor.model_dump()
    
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
            sensor_data = SensorUpdate(**data)
        except PydanticValidationError as e:
            raise ValidationError(f"Invalid input data: {str(e)}")
        
        db_data = {k: v for k, v in sensor_data.model_dump(mode='python').items() if v is not None}
        
        if not db_data:
            raise ValidationError("No fields to update")
        
        if 'type' in db_data:
            db_data['type'] = str(db_data['type'].value)
        
        updated_sensor = self.repository.update_sensor(sensor_id, db_data)
        data = self.get_data_from_statemon(sensor_id)
        sensor = SensorResponse(
            **updated_sensor, 
            value=data.get('value', None), 
            unit=data.get('unit', None), 
            status=data.get('status', None)
        )
        return sensor.model_dump()
    
    def delete_sensor(self, sensor_id: int) -> bool:
        """Удалить сенсор"""
        if not isinstance(sensor_id, int) or sensor_id <= 0:
            raise ValidationError("Invalid sensor ID")
        
        return self.repository.delete_sensor(sensor_id)