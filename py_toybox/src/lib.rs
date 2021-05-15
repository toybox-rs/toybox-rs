use pyo3::{
    exceptions::{self, PyValueError},
    prelude::*,
    types::PyByteArray,
};
use toybox::{self, graphics::ImageBuffer, Simulation};
use toybox_core::AleAction;

#[pymodule]
fn py_toybox(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Game>()?;
    m.add_class::<State>()?;
    m.add_class::<Input>()?;

    Ok(())
}

#[pyclass]
struct Game {
    inner: Box<dyn Simulation + Send>,
}

#[pymethods]
impl Game {
    #[new]
    fn new(name: &str) -> PyResult<Game> {
        let inner = toybox::get_simulation_by_name(name)
            .map_err(|e| exceptions::PyLookupError::new_err(e))?;
        Ok(Self { inner })
    }

    fn from_json(&self, json_str: &str) -> PyResult<Game> {
        Ok(Self {
            inner: self
                .inner
                .from_json(json_str)
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?,
        })
    }

    fn seed(&mut self, seed: u32) -> PyResult<()> {
        self.inner.as_mut().reset_seed(seed);
        Ok(())
    }

    fn to_json(&self) -> PyResult<String> {
        Ok(self.inner.to_json())
    }

    fn config_schema(&self) -> PyResult<String> {
        Ok(self.inner.schema_for_config())
    }
    fn frame_schema(&self) -> PyResult<String> {
        Ok(self.inner.schema_for_state())
    }

    fn is_legal(&self, action: i32) -> PyResult<bool> {
        let actions = self.inner.legal_action_set();
        if let Some(action) = AleAction::from_int(action) {
            Ok(actions.contains(&action))
        } else {
            Ok(false)
        }
    }

    fn frame_size(&self) -> PyResult<(i32, i32)> {
        Ok(self.inner.game_size())
    }

    fn legal_actions(&self) -> PyResult<Vec<i32>> {
        Ok(self
            .inner
            .legal_action_set()
            .into_iter()
            .map(|x| x.to_int())
            .collect())
    }

    fn new_game(&mut self) -> PyResult<State> {
        Ok(State {
            shape: self.inner.game_size(),
            inner: self.inner.as_mut().new_game(),
        })
    }
    fn new_state(&self, json_str: &str) -> PyResult<State> {
        let state = self
            .inner
            .new_state_from_json(json_str)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        Ok(State {
            shape: self.inner.game_size(),
            inner: state,
        })
    }
}

#[pyclass]
#[derive(Default, Clone)]
struct Input {
    #[pyo3(get, set)]
    pub left: bool,
    #[pyo3(get, set)]
    pub right: bool,
    #[pyo3(get, set)]
    pub up: bool,
    #[pyo3(get, set)]
    pub down: bool,
    #[pyo3(get, set)]
    pub button1: bool,
    #[pyo3(get, set)]
    pub button2: bool,
}

#[pymethods]
impl Input {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self::default())
    }
}

#[pyclass]
struct State {
    shape: (i32, i32),
    inner: Box<dyn toybox::State + Send>,
}

#[pymethods]
impl State {
    fn copy(&self) -> PyResult<Self> {
        Ok(Self {
            shape: self.shape.clone(),
            inner: self.inner.copy(),
        })
    }
    fn apply_ale_action(&mut self, action: i32) -> PyResult<bool> {
        if let Some(action) = AleAction::from_int(action) {
            self.inner.as_mut().update_mut(action.to_input());
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn render_into_buffer(&self, buffer: &PyByteArray) -> PyResult<()> {
        let (w, h) = self.shape;
        let mut img = ImageBuffer::alloc(w, h);
        img.render(&self.inner.draw());
        let dest: &mut [u8] = unsafe { buffer.as_bytes_mut() };
        for (src, dest) in img.data.iter().cloned().zip(dest.iter_mut()) {
            *dest = src;
        }
        Ok(())
    }
    fn apply_action(&mut self, input: &PyCell<Input>) -> PyResult<()> {
        let input: PyRef<Input> = input.borrow();
        let tb_input = toybox_core::Input {
            left: input.left,
            right: input.right,
            up: input.up,
            down: input.down,
            button1: input.button1,
            button2: input.button2,
        };
        self.inner.as_mut().update_mut(tb_input);
        Ok(())
    }

    fn game_over(&self) -> PyResult<bool> {
        Ok(self.inner.lives() < 0)
    }
    fn lives(&self) -> PyResult<i32> {
        Ok(self.inner.lives())
    }
    fn level(&self) -> PyResult<i32> {
        Ok(self.inner.level())
    }
    fn score(&self) -> PyResult<i32> {
        Ok(self.inner.score())
    }
    fn to_json(&self) -> PyResult<String> {
        Ok(self.inner.to_json())
    }
    fn query(&self, query: &str, options: Option<&str>) -> PyResult<String> {
        let value = if let Some(opt_json) = options {
            serde_json::from_str(opt_json).map_err(|e| PyValueError::new_err(format!("{:?}", e)))?
        } else {
            serde_json::Value::Null
        };
        Ok(self
            .inner
            .query_json(query, &value)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?)
    }
}
