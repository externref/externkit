from typing import List, Optional, Tuple, Union, Any

class SqliteClient:
    """SQLite database client for Python."""
    
    def __init__(self, connection_string: str) -> None:
        """Initialize SQLite connection.
        
        Args:
            connection_string: Path to SQLite database file or ':memory:' for in-memory database
        """
        ...
    
    def query(self, query: str, params: List[Any] = ...) -> List[Union[Any, Tuple[Any, ...]]]:
        """Execute a SQL query and return results.
        
        Args:
            query: SQL query string with ? placeholders
            params: List of parameters to bind to the query
            
        Returns:
            List of results. Single column returns scalar values, multiple columns return tuples
        """
        ...
    
    def create_table(self, table_name: str, columns: List[Tuple[str, str]]) -> None:
        """Create a table if it doesn't exist.
        
        Args:
            table_name: Name of the table to create
            columns: List of (column_name, column_type) tuples
        """
        ...
    
    def insert(self, table_name: str, columns: List[str], values: List[Any]) -> None:
        """Insert a record into the table.
        
        Args:
            table_name: Name of the table
            columns: List of column names
            values: List of values to insert
        """
        ...
    
    def select(
        self, 
        table_name: str, 
        columns: List[str], 
        where_clause: Optional[str] = None
    ) -> List[Union[Any, Tuple[Any, ...]]]:
        """Select records from the table.
        
        Args:
            table_name: Name of the table
            columns: List of column names to select
            where_clause: Optional WHERE clause (without the WHERE keyword)
            
        Returns:
            List of results. Single column returns scalar values, multiple columns return tuples
        """
        ...
    
    def update(self, table_name: str, set_clause: str, where_clause: str) -> int:
        """Update records in the table.
        
        Args:
            table_name: Name of the table
            set_clause: SET clause (without the SET keyword)
            where_clause: WHERE clause (without the WHERE keyword)
            
        Returns:
            Number of affected rows
        """
        ...
    
    def delete(self, table_name: str, where_clause: str) -> int:
        """Delete records from the table.
        
        Args:
            table_name: Name of the table
            where_clause: WHERE clause (without the WHERE keyword)
            
        Returns:
            Number of affected rows
        """
        ...
    
    def close(self) -> None:
        """Close the database connection."""
        ...
