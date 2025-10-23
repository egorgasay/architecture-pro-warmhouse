"""Pydantic модели для валидации данных сенсоров"""

from datetime import datetime
from typing import Optional
from pydantic import BaseModel, Field, field_validator
from enum import Enum


class SensorType(str, Enum):
    """Допустимые типы сенсоров"""
    TEMPERATURE = "temperature"


class SensorStatus(str, Enum):
    """Статусы сенсора"""
    ACTIVE = "active"
    INACTIVE = "inactive"
    ERROR = "error"
    MAINTENANCE = "maintenance"


class SensorBase(BaseModel):
    """Базовая модель сенсора"""
    name: str = Field(..., min_length=1, max_length=100, description="Название сенсора")
    type: SensorType = Field(..., description="Тип сенсора")
    location: str = Field(..., min_length=1, max_length=200, description="Местоположение сенсора")
    unit: Optional[str] = Field(None, min_length=1, max_length=20, description="Единица измерения")


class SensorCreate(SensorBase):
    """Модель для создания сенсора"""
    pass
    


class SensorUpdate(BaseModel):
    """Модель для обновления сенсора (все поля опциональны)"""
    name: Optional[str] = Field(None, min_length=1, max_length=100)
    type: Optional[SensorType] = None
    location: Optional[str] = Field(None, min_length=1, max_length=200)
    unit: Optional[str] = Field(None, min_length=1, max_length=20)


class SensorResponse(BaseModel):
    """Модель ответа с полной информацией о сенсоре"""
    id: int
    name: str = Field(..., min_length=1, max_length=100, description="Название сенсора")
    type: SensorType = Field(..., description="Тип сенсора")
    location: str = Field(..., min_length=1, max_length=200, description="Местоположение сенсора")
    unit: Optional[str] = Field(None, min_length=1, max_length=20, description="Единица измерения")
    value: Optional[float] = Field(None, description="Текущее значение сенсора")
    status: Optional[SensorStatus] = Field(None, description="Статус сенсора")
    last_updated: datetime
    created_at: datetime
    
    class Config:
        from_attributes = True

