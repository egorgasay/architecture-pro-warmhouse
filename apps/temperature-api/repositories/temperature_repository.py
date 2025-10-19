import random
from abc import ABC, abstractmethod


class TemperatureRepository(ABC):
    """Абстрактный репозиторий для работы с температурными данными"""
    
    @abstractmethod
    def get_random_temperature(self) -> float:
        pass


class RandomTemperatureRepository(TemperatureRepository):
    """Репозиторий, возвращающий случайные значения температуры"""
    
    def __init__(self, min_temp: float = -20.0, max_temp: float = 40.0):
        self.min_temp = min_temp
        self.max_temp = max_temp
    
    def get_random_temperature(self) -> float:
        """Возвращает случайное значение температуры в заданном диапазоне"""
        return round(random.uniform(self.min_temp, self.max_temp), 1)
