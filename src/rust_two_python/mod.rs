pub mod rust_two_python {
    use pyo3::prelude::*;
    use pyo3::types::{ IntoPyDict, PyList };
    use std::fmt::Error;
    use serde_json::{ Value, Map, json };
    use std::collections::{ HashMap, HashSet };

    fn parse_serde_array(input_val: Vec<Value>, py: Python<'_>) -> Vec<PyObject> {
        let mut input_obj: Vec<PyObject> = Vec::new();
        for i in input_val {
            if i.is_i64() {
                let num = i.as_i64().unwrap() as i64;
                input_obj.push(num.to_object(py));
            } else if i.is_f64() {
                let num = i.as_f64().unwrap() as i64;
                input_obj.push(num.to_object(py));
            } else if i.is_boolean() {
                let bool = i.as_bool().unwrap();
                input_obj.push(bool.to_object(py));
            } else if i.is_string() {
                let string_val = i.as_str().unwrap();
                input_obj.push(string_val.to_object(py));
            } else if i.is_object() {
                let object_val = parse_serde_obj(i.as_object().unwrap().clone(), py);
                input_obj.push(object_val.to_object(py));
            } else if i.is_null() {
                input_obj.push(i.as_null().to_object(py));
            } else if i.is_array() {
                let array_val = parse_serde_array(i.as_array().unwrap().clone(), py);
                input_obj.push(array_val.to_object(py));
            }
        }
        return input_obj;
    }
    fn parse_serde_obj(input_obj: Map<String, Value>, py: Python<'_>) -> HashMap<String, PyObject> {
        let mut output_map = HashMap::<String, PyObject>::new();
        for i in input_obj.keys() {
            if input_obj.get(i).unwrap().is_i64() {
                let num = input_obj.get(i).unwrap().as_i64().unwrap() as i64;
                output_map.insert(String::from(i), num.to_object(py));
            } else if input_obj.get(i).unwrap().is_f64() {
                let num = input_obj.get(i).unwrap().as_f64().unwrap() as f64;
                output_map.insert(String::from(i), num.to_object(py));
            } else if input_obj.get(i).unwrap().is_boolean() {
                println!("Test1");
                let bool_val = input_obj.get(i).unwrap().as_bool();
                output_map.insert(String::from(i), bool_val.to_object(py));
            } else if input_obj.get(i).unwrap().is_string() {
                let string_val = input_obj.get(i).unwrap().as_str();
                output_map.insert(String::from(i), string_val.to_object(py));
            } else if input_obj.get(i).unwrap().is_object() {
                output_map.insert(
                    String::from(i),
                    parse_serde_obj(
                        input_obj.get(i).unwrap().as_object().unwrap().clone(),
                        py
                    ).to_object(py)
                );
            } else if input_obj.get(i).unwrap().is_null() {
                let val = input_obj.get(i).unwrap().as_null().to_object(py);
                output_map.insert(String::from(i), val);
            } else if input_obj.get(i).unwrap().is_array() {
                let val = input_obj.get(i).unwrap().as_array().unwrap();
                output_map.insert(
                    String::from(i),
                    parse_serde_array(val.to_vec(), py).to_object(py)
                );
            }
        }
        return output_map;
    }
    fn parse_python_output(result: Py<PyAny>, py: Python<'_>) -> Value {
        if let Ok(_bool) = result.downcast_bound::<pyo3::types::PyBool>(py) {
            let val: bool = result.extract(py).unwrap();
            return Value::Bool(val);
        } else if let Ok(_int) = result.downcast_bound::<pyo3::types::PyInt>(py) {
            let val: i64 = result.extract(py).unwrap();
            return serde_json::json!(val);
        } else if let Ok(_f64) = result.downcast_bound::<pyo3::types::PyFloat>(py) {
            let val: f64 = result.extract(py).unwrap();
            return serde_json::json!(val);
        } else if let Ok(_string) = result.downcast_bound::<pyo3::types::PyString>(py) {
            let val: String = result.extract(py).unwrap();
            return Value::String(val);
        } else if let Ok(_dict) = result.downcast_bound::<pyo3::types::PyDict>(py) {
            let val: HashMap<String, PyObject> = result.extract(py).unwrap();
            let mut output_dict: HashMap<String, Value> = HashMap::new();
            for i in val.keys() {
                let cur_py_obj = val.get(i).unwrap().clone();
                output_dict.insert(i.to_string(), parse_python_output(cur_py_obj, py));
            }
            return serde_json::json!(output_dict);
        } else if let Ok(_pylist) = result.downcast_bound::<pyo3::types::PyList>(py) {
            let val: Vec<PyObject> = result.extract(py).unwrap();
            let mut output_vec: Vec<Value> = Vec::new();
            for i in val {
                let cur_py_obj = i.clone();
                output_vec.push(parse_python_output(cur_py_obj, py));
            }
            return serde_json::json!(output_vec);
        } else if let Ok(_pyunicode) = result.downcast_bound::<pyo3::types::PyUnicode>(py) {
            let val: String = result.extract(py).unwrap();
            return Value::String(val);
        } else if let Ok(_pybytes) = result.downcast_bound::<pyo3::types::PyBytes>(py) {
            let val: Vec<u8> = result.extract(py).unwrap();
            return json!(val);
        } else if let Ok(_pybytes) = result.downcast_bound::<pyo3::types::PyTuple>(py) {
            let val: Vec<PyObject> = result.extract(py).unwrap();
            let mut output_vec: Vec<Value> = Vec::new();
            for i in val {
                let cur_py_obj = i.clone();
                output_vec.push(parse_python_output(cur_py_obj, py));
            }
            return serde_json::json!(output_vec);
        } else if let Ok(_pybytes) = result.downcast_bound::<pyo3::types::PySet>(py) {
            let val: HashSet<String> = result.extract(py).unwrap();
            return serde_json::json!(val);
        } else if let Ok(_pyfrosenset) = result.downcast_bound::<pyo3::types::PyFrozenSet>(py) {
            let val: HashSet<String> = result.extract(py).unwrap();
            return serde_json::json!(val);
        } else if let Ok(_pybytes) = result.downcast_bound::<pyo3::types::PyByteArray>(py) {
            let val: Vec<u8> = result.extract(py).unwrap();
            return json!(val);
        } else if let Ok(_pydatetime) = result.downcast_bound::<pyo3::types::PyDateTime>(py) {
            let val: chrono::NaiveDateTime = result.extract(py).unwrap();
            return json!(val.to_string());
        } else if let Ok(_pydate) = result.downcast_bound::<pyo3::types::PyDate>(py) {
            let val: chrono::NaiveDate = result.extract(py).unwrap();
            return json!(val.to_string());
        } else if let Ok(_pytime) = result.downcast_bound::<pyo3::types::PyTime>(py) {
            let val: chrono::NaiveTime = result.extract(py).unwrap();
            return json!(val.to_string());
        } else if let Ok(_pytzinfo) = result.downcast_bound::<pyo3::types::PyTzInfo>(py) {
            let val: chrono::Utc = result.extract(py).unwrap();
            return json!(val.to_string());
        } else if let Ok(_pydelta) = result.downcast_bound::<pyo3::types::PyDelta>(py) {
            let val: chrono::Duration = result.extract(py).unwrap();
            return json!(val.to_string());
        }
        return Value::Null;
    }

    pub fn execute_python_function(
        path_str: impl AsRef<Path>,
        module_name: String,
        function_name: String,
        args: Value
    ) -> Value {
        Python::with_gil(|py| {
            let path: String = String::from(path_str.as_ref().as_os_str().to_str().unwrap());
            let syspath = py
                .import_bound("sys")
                .unwrap()
                .getattr("path")
                .unwrap()
                .downcast_into::<PyList>()
                .unwrap();
            syspath.insert(0, &path).unwrap();
            let value_obj = args.as_object().unwrap();
            let mut kwargs = HashMap::<String, PyObject>::new();
            kwargs = parse_serde_obj(value_obj.clone(), py);
            let module = py.import_bound(module_name.as_str()).unwrap();
            let fun: Py<PyAny> = module.getattr(function_name.as_str()).unwrap().into();
            let res: Py<PyAny> = fun
                .call_bound(py, (), Some(&kwargs.into_py_dict_bound(py)))
                .unwrap();
            return parse_python_output(res, py);
        })
    }
}
#[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;
        use std::{ env, vec };

        #[test]
        fn test_add() {
            let path = env::current_dir().unwrap();
            let path_str = path.as_os_str().to_str().unwrap();
            let val = serde_json::from_str(r#"{"x":10,"y":20}"#).unwrap();
            let res = rust_two_python::execute_python_function(
                vec![String::from(path.to_str().unwrap())],
                String::from("test"),
                String::from("add_num"),
                val
            ).unwrap();
            let result_obj = res.as_object().unwrap();
            assert_eq!(result_obj.get("result").unwrap().as_i64().unwrap(), 30);
        }
        #[test]
        fn test_sub() {
            let path = env::current_dir().unwrap();
            let path_str = path.as_os_str().to_str().unwrap();
            let val = serde_json::from_str(r#"{"x":10,"y":20}"#).unwrap();
            let res = rust_two_python::execute_python_function(
                vec![String::from(path.to_str().unwrap())],
                String::from("test"),
                String::from("sub_num"),
                val
            ).unwrap();
            let result_obj = res.as_object().unwrap();
            assert_eq!(result_obj.get("result").unwrap().as_i64().unwrap(), -10);
        }

        #[test]
        fn test_mul() {
            let path = env::current_dir().unwrap();
            let path_str = path.as_os_str().to_str().unwrap();
            let val = serde_json::from_str(r#"{"x":10,"y":10}"#).unwrap();
            let res = rust_two_python::execute_python_function(
                vec![String::from(path.to_str().unwrap())],
                String::from("test"),
                String::from("mul_num"),
                val
            ).unwrap();
            let result_obj = res.as_object().unwrap();
            assert_eq!(result_obj.get("result").unwrap().as_i64().unwrap(), 100);
        }

        #[test]
        fn test_div() {
            let path = std::env::current_dir().unwrap();
            let path_str = path.as_os_str().to_str().unwrap();
            let val = serde_json::from_str(r#"{"x":1,"y":2}"#).unwrap();
            let res = rust_two_python::execute_python_function(
                vec![String::from(path.to_str().unwrap())],
                String::from("test"),
                String::from("div_num"),
                val
            ).unwrap();
            let result_obj = res.as_object().unwrap();
            assert_eq!(result_obj.get("result").unwrap().as_f64().unwrap(), 0.5);
        }
        #[test]
        fn test_write_file() {
            let path = std::env::current_dir().unwrap();
            let val = serde_json::from_str(r#"{"test_str":"Hallo Welt"}"#).unwrap();
            let res = rust_two_python::execute_python_function(
                vec![String::from(path.to_str().unwrap())],
                String::from("test"),
                String::from("write_file"),
                val
            );
        }
    }
