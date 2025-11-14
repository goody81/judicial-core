use pyo3::prelude::*;
use crate::{JudicialCore, SystemAction, Verdict};

#[pyclass]
pub struct PyJudicialCore {
    core: JudicialCore,
}

#[pymethods]
impl PyJudicialCore {
    #[new]
    pub fn new() -> Self {
        PyJudicialCore {
            core: JudicialCore::new(),
        }
    }

    pub fn rule(&self, action_type: String, payload: String, context: String) -> PyResult<String> {
        let action = SystemAction {
            action_type,
            payload,
            context,
        };

        let verdict = self.core.rule(action);
        
        match verdict {
            Verdict::Approved => Ok("APPROVED".to_string()),
            Verdict::Rejected(reason) => Ok(format!("REJECTED: {}", reason)),
            Verdict::RejectedWithSuggestion(reason, suggestion) => {
                Ok(format!("REJECTED_WITH_SUGGESTION: {} | {}", reason, suggestion))
            }
        }
    }

    pub fn get_compliance_score(&self) -> f64 {
        self.core.get_compliance_score()
    }

    pub fn export_ledger(&self) -> String {
        self.core.export_ledger()
    }
}

#[pymodule]
fn _judicial(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJudicialCore>()?;
    Ok(())
}
