use pyo3::prelude::*;

fn load_env_vars() -> std::collections::HashMap<String, String> {
    let content =
        std::fs::read_to_string(".externkit/environment_variables.json").unwrap_or_default();
    if content.is_empty() {
        std::collections::HashMap::new()
    } else {
        serde_json::from_str(&content).unwrap_or_default()
    }
}

#[pyfunction]
fn get(env_name: String) -> Option<String> {
    match std::env::var(&env_name) {
        Ok(val) => Some(val),
        Err(_) => load_env_vars().get(&env_name).cloned(),
    }
}

#[pymodule]
fn externkit(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let env_module = PyModule::new(m.py(), "env")?;
    env_module.add_function(wrap_pyfunction!(get, &env_module)?)?;
    m.add_submodule(&env_module)?;
    Ok(())
}
