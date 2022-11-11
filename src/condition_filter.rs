use core::fmt;

use quick_js::Context;

pub struct ConditionFilter {
    context: Context,
    condition: String,
}

impl ConditionFilter {
    pub fn new<S>(condition: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            context: Context::new().unwrap(),
            condition: condition.into(),
        }
    }
    pub fn filter(&self, line: &str) -> bool {
        self.context
            .eval_as::<bool>(
                format!("(function (){{return ({})}}).call({line})", self.condition).as_str(),
            )
            .unwrap()
    }
}

impl fmt::Debug for ConditionFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConditionFilter [`{}`]", self.condition)
    }
}

impl Clone for ConditionFilter {
    fn clone(&self) -> Self {
        Self {
            context: Context::new().unwrap(),
            condition: self.condition.clone(),
        }
    }
}
