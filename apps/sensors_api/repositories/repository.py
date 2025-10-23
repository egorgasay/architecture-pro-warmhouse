"""Репозиторий для работы с PostgreSQL"""

import psycopg
from psycopg import errors as psycopg_errors
from psycopg_pool import ConnectionPool
from typing import Optional, List, Dict, Any
from contextlib import contextmanager
from exceptions import DatabaseError, NotFoundError


class PostgresRepository:
    """Репозиторий с connection pooling и обработкой ошибок"""
    
    def __init__(self, db_url: str, min_conn: int = 1, max_conn: int = 10):
        """
        Инициализация пула подключений
        
        Args:
            db_url: URL подключения к БД
            min_conn: Минимальное количество подключений
            max_conn: Максимальное количество подключений
        """
        try:
            self.pool = ConnectionPool(db_url, min_size=min_conn, max_size=max_conn)
        except psycopg_errors.OperationalError as e:
            raise DatabaseError(f"Database connection failed: {str(e)}")
    
    @contextmanager
    def get_cursor(self):
        """Context manager для безопасной работы с курсором"""
        conn = None
        cursor = None
        try:
            conn = self.pool.getconn()
            cursor = conn.cursor()
            yield cursor
            conn.commit()
        except psycopg_errors.Error as e:
            if conn:
                conn.rollback()
            raise DatabaseError(f"Database operation failed: {str(e)}")
        finally:
            if cursor:
                cursor.close()
            if conn:
                self.pool.putconn(conn)
    
    def get_sensors(self) -> List[Dict[str, Any]]:
        """Получить все сенсоры"""
        try:
            with self.get_cursor() as cursor:
                cursor.execute(
                    "SELECT id, name, type, location, value, unit, status, last_updated, created_at FROM sensors ORDER BY id"
                )
                rows = cursor.fetchall()
                return [self._row_to_dict(row) for row in rows]
        except DatabaseError:
            raise
        except Exception as e:
            raise DatabaseError(f"Failed to fetch sensors: {str(e)}")
    
    def create_sensor(self, data: dict) -> Dict[str, Any]:
        """
        Создать новый сенсор
        
        Args:
            data: Словарь с данными сенсора
            
        Returns:
            Словарь с созданным сенсором
        """
        try:
            with self.get_cursor() as cursor:
                cursor.execute(
                    """
                    INSERT INTO sensors (
                        name, type, location, value, unit, status, last_updated, created_at
                    ) VALUES (
                        %s, %s, %s, %s, %s, %s, %s, %s
                    )
                    RETURNING id, name, type, location, value, unit, status, last_updated, created_at
                    """,
                    (
                        data['name'], 
                        data['type'], 
                        data['location'], 
                        data['value'], 
                        data['unit'], 
                        data['status'], 
                        data['last_updated'], 
                        data['created_at']
                    )
                )
                row = cursor.fetchone()
                if not row:
                    raise DatabaseError("Failed to create sensor")
                return self._row_to_dict(row)
        except DatabaseError:
            raise
        except Exception as e:
            raise DatabaseError(f"Failed to create sensor: {str(e)}")
    
    def get_sensor_by_id(self, sensor_id: int) -> Dict[str, Any]:
        """
        Получить сенсор по ID
        
        Args:
            sensor_id: ID сенсора
            
        Returns:
            Словарь с данными сенсора
            
        Raises:
            NotFoundError: Если сенсор не найден
        """
        try:
            with self.get_cursor() as cursor:
                cursor.execute(
                    "SELECT id, name, type, location, value, unit, status, last_updated, created_at FROM sensors WHERE id = %s",
                    (sensor_id,)
                )
                row = cursor.fetchone()
                if not row:
                    raise NotFoundError(f"Sensor with id {sensor_id} not found")
                return self._row_to_dict(row)
        except (DatabaseError, NotFoundError):
            raise
        except Exception as e:
            raise DatabaseError(f"Failed to fetch sensor: {str(e)}")
    
    def update_sensor(self, sensor_id: int, data: dict) -> Dict[str, Any]:
        """
        Обновить сенсор
        
        Args:
            sensor_id: ID сенсора
            data: Словарь с обновленными данными
            
        Returns:
            Словарь с обновленным сенсором
        """
        # Формируем динамический запрос только с переданными полями
        update_fields = []
        values = []
        
        for key in ['name', 'type', 'location', 'value', 'unit', 'status']:
            if key in data:
                update_fields.append(f"{key} = %s")
                values.append(data[key])
        
        if not update_fields:
            raise DatabaseError("No fields to update")
        
        # Обновляем last_updated
        update_fields.append("last_updated = CURRENT_TIMESTAMP")
        values.append(sensor_id)
        
        try:
            with self.get_cursor() as cursor:
                query = f"""
                    UPDATE sensors 
                    SET {', '.join(update_fields)}
                    WHERE id = %s
                    RETURNING id, name, type, location, value, unit, status, last_updated, created_at
                """
                cursor.execute(query, values)
                row = cursor.fetchone()
                if not row:
                    raise NotFoundError(f"Sensor with id {sensor_id} not found")
                return self._row_to_dict(row)
        except (DatabaseError, NotFoundError):
            raise
        except Exception as e:
            raise DatabaseError(f"Failed to update sensor: {str(e)}")
    
    def delete_sensor(self, sensor_id: int) -> bool:
        """
        Удалить сенсор
        
        Args:
            sensor_id: ID сенсора
            
        Returns:
            True если удален успешно
        """
        try:
            with self.get_cursor() as cursor:
                cursor.execute("DELETE FROM sensors WHERE id = %s RETURNING id", (sensor_id,))
                row = cursor.fetchone()
                if not row:
                    raise NotFoundError(f"Sensor with id {sensor_id} not found")
                return True
        except (DatabaseError, NotFoundError):
            raise
        except Exception as e:
            raise DatabaseError(f"Failed to delete sensor: {str(e)}")
    
    @staticmethod
    def _row_to_dict(row: tuple) -> Dict[str, Any]:
        """Преобразование строки БД в словарь"""
        return {
            'id': row[0],
            'name': row[1],
            'type': row[2],
            'location': row[3],
            'value': row[4],
            'unit': row[5],
            'status': row[6],
            'last_updated': row[7],
            'created_at': row[8]
        }
    
    def close(self):
        """Закрыть все подключения в пуле"""
        if self.pool:
            self.pool.closeall()