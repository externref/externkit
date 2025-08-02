use pyo3::prelude::*;

use std::sync::{Arc, Mutex};

#[pyclass]
pub struct SqliteClient {
    client: Arc<Mutex<rusqlite::Connection>>,
}

#[pymethods]
impl SqliteClient {
    #[new]
    fn new(connection_string: &str) -> Self {
        let connection = rusqlite::Connection::open(connection_string).unwrap();    
        SqliteClient {
            client: Arc::new(Mutex::new(connection)),
        }
    }

    fn query(&self, py: Python<'_>, query: &str, params: Vec<PyObject>) -> PyResult<Vec<PyObject>> {
        let conn = self.client.lock().unwrap();
        let mut stmt = conn.prepare(query).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        let converted_params: Vec<String> = params
            .iter()
            .map(|obj| obj.extract::<String>(py).unwrap_or_else(|_| "".to_string()))
            .collect();
        
        let param_refs: Vec<&dyn rusqlite::ToSql> = converted_params
            .iter()
            .map(|s| s as &dyn rusqlite::ToSql)
            .collect();
        
        let rows = stmt.query_map(&param_refs[..], |row| {
            let column_count = row.as_ref().column_count();
            let mut row_values = Vec::new();
            
            for i in 0..column_count {
                let value: rusqlite::types::Value = row.get(i)?;
                row_values.push(value);
            }
            
            Ok(row_values)
        }).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        let mut result = Vec::new();
        for row in rows {
            let row_values = row.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
            
            if row_values.len() == 1 {
                // Single column - return the value directly (backward compatibility)
                let py_value = match &row_values[0] {
                    rusqlite::types::Value::Null => py.None(),
                    rusqlite::types::Value::Integer(i) => pyo3::types::PyInt::new(py, *i).into(),
                    rusqlite::types::Value::Real(f) => pyo3::types::PyFloat::new(py, *f).into(),
                    rusqlite::types::Value::Text(t) => pyo3::types::PyString::new(py, t).into(),
                    rusqlite::types::Value::Blob(b) => pyo3::types::PyBytes::new(py, b).into(),
                };
                result.push(py_value);
            } else {
                // Multiple columns - return as a list/tuple
                let mut py_row = Vec::new();
                for value in row_values {
                    let py_value = match value {
                        rusqlite::types::Value::Null => py.None(),
                        rusqlite::types::Value::Integer(i) => pyo3::types::PyInt::new(py, i).into(),
                        rusqlite::types::Value::Real(f) => pyo3::types::PyFloat::new(py, f).into(),
                        rusqlite::types::Value::Text(t) => pyo3::types::PyString::new(py, &t).into(),
                        rusqlite::types::Value::Blob(b) => pyo3::types::PyBytes::new(py, &b).into(),
                    };
                    py_row.push(py_value);
                }
                let py_tuple = pyo3::types::PyTuple::new(py, py_row).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
                result.push(py_tuple.into());
            }
        }
        Ok(result)
    }

    fn create_table(
        &self,
        table_name: &str,
        columns: Vec<(String, String)>,
    ) -> PyResult<()> {
        let conn = self.client.lock().unwrap();
        let columns_def: Vec<String> = columns
            .iter()
            .map(|(name, type_)| format!("{} {}", name, type_))
            .collect();
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            table_name,
            columns_def.join(", ")
        );
        conn.execute(&sql, []).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        Ok(())
    }

    fn insert(&self, py: Python<'_>, table_name: &str, columns: Vec<String>, values: Vec<PyObject>) -> PyResult<()> {
        let conn = self.client.lock().unwrap();
        let converted_values: Vec<String> = values
            .iter()
            .map(|obj| obj.extract::<String>(py).unwrap_or_else(|_| "".to_string()))
            .collect();
        
        let columns_str = columns.join(", ");
        let placeholders = values.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", table_name, columns_str, placeholders);
        
        let param_refs: Vec<&dyn rusqlite::ToSql> = converted_values
            .iter()
            .map(|s| s as &dyn rusqlite::ToSql)
            .collect();
        
        conn.execute(&sql, &param_refs[..]).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        Ok(())
    }

    fn select(&self, py: Python<'_>, table_name: &str, columns: Vec<String>, where_clause: Option<String>) -> PyResult<Vec<PyObject>> {
        let columns_str = columns.join(", ");
        let sql = match where_clause {
            Some(clause) => format!("SELECT {} FROM {} WHERE {}", columns_str, table_name, clause),
            None => format!("SELECT {} FROM {}", columns_str, table_name),
        };
        self.query(py, &sql, vec![])
    }

    fn delete(&self, table_name: &str, where_clause: &str) -> PyResult<usize> {
        let conn = self.client.lock().unwrap();
        let sql = format!("DELETE FROM {} WHERE {}", table_name, where_clause);
        let rows_affected = conn.execute(&sql, []).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        Ok(rows_affected)
    }

    fn update(&self, table_name: &str, set_clause: &str, where_clause: &str) -> PyResult<usize> {
        let conn = self.client.lock().unwrap();
        let sql = format!("UPDATE {} SET {} WHERE {}", table_name, set_clause, where_clause);
        let rows_affected = conn.execute(&sql, []).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        Ok(rows_affected)
    }

    fn close(&mut self) -> PyResult<()> {
        Ok(())
    }
}   