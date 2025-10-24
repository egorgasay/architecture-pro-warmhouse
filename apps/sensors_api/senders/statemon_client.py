"""Клиент для получения данных из state_monitoring_api"""

import requests

class StateMonitoringClient:
    """Клиент для получения данных из state_monitoring_api"""

    def __init__(self, base_url: str):
        self.base_url = base_url

    def get_data(self, sensor_id: int):
        """Получение данных из state_monitoring_api"""
        response = requests.get(f"{self.base_url}/api/v1/sensor/data?sensor_id={sensor_id}")
        if response.status_code != 200:
            raise Exception(f"Failed to get data from state_monitoring_api: {response.status_code} {response.text}")
        return response.json() 