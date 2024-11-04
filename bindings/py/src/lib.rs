use pyo3::prelude::*;
/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
//use bncsv_core::functional::fmt::utf8::Utf8Converter;

#[pymodule]
mod rs_api {

    use bncsv_core::{
        compr::BnCsvConverter, fmt::utf8::Utf8Converter, utils::iterators::TryChunks,
    };
    use pyo3::{
        exceptions,
        prelude::*,
        types::{PyBytes, PyIterator},
    };

    #[pyfunction]
    fn encode(py: Python<'_>, input: Py<PyIterator>, writer: Py<PyAny>) -> PyResult<()> {
        let data_iter = input.bind(py).clone().map(|el| {
            el.expect("Failed to read input")
                .extract::<u8>()
                .expect("Cannot read input byte")
        });
        Utf8Converter::encode(data_iter)
            .try_chunks(4096)
            .try_for_each(|x| match x {
                Ok(chunk) => {
                    let py_bytes = PyBytes::new_bound(py, &chunk);
                    writer
                        .call_method1(py, "write", (py_bytes,))
                        .expect("Failed to write to output");
                    Ok(())
                }
                Err(e) => Err(exceptions::PyBufferError::new_err(e.to_string())),
            })
    }
    #[pyfunction]
    fn decode(py: Python<'_>, input: Py<PyIterator>, writer: Py<PyAny>) -> PyResult<()> {
        let data_iter = input.bind(py).clone().map(|el| {
            el.expect("Failed to read input")
                .extract::<u8>()
                .expect("Cannot read input byte")
        });
        Utf8Converter::decode(data_iter)
            .try_chunks(4096)
            .try_for_each(|x| match x {
                Ok(chunk) => {
                    let py_bytes = PyBytes::new_bound(py, &chunk);
                    writer
                        .call_method1(py, "write", (py_bytes,))
                        .expect("Failed to write to input");
                    Ok(())
                }
                Err(e) => Err(exceptions::PyBufferError::new_err(e.to_string())),
            })
    }
}
