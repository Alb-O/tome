use std::sync::{Mutex, OnceLock};

use embed_nu::{CommandGroupConfig, Context, PipelineData, Value};
use nu_protocol::{Config, ShellError, Span};
use tome_core::{Rope, Selection};

#[derive(Debug, Clone)]
pub enum NuError {
    Disabled(String),
    Eval(String),
}

impl std::fmt::Display for NuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NuError::Disabled(msg) => write!(f, "{msg}"),
            NuError::Eval(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for NuError {}

impl From<ShellError> for NuError {
    fn from(value: ShellError) -> Self {
        NuError::Eval(value.to_string())
    }
}

#[derive(Debug, Clone, Default)]
pub struct CompletionItem {
    pub value: String,
}

pub struct NuEngine {
    ctx: Mutex<Context>,
    config: Config,
}

static ENGINE: OnceLock<Result<NuEngine, NuError>> = OnceLock::new();

impl NuEngine {
    pub fn global() -> Result<&'static Self, NuError> {
        match ENGINE.get_or_init(NuEngine::initialize) {
            Ok(engine) => Ok(engine),
            Err(err) => Err(err.clone()),
        }
    }

    fn initialize() -> Result<NuEngine, NuError> {
        let ctx = Context::builder()
            .with_command_groups(CommandGroupConfig::default().all_groups(true))
            .map_err(|e| NuError::Eval(e.to_string()))?
            .add_parent_env_vars()
            .build()
            .map_err(|e| NuError::Eval(e.to_string()))?;

        Ok(NuEngine {
            ctx: Mutex::new(ctx),
            config: Config::default(),
        })
    }


    pub fn run(&self, script: &str, input: PipelineData) -> Result<PipelineData, NuError> {
        let mut ctx = self
            .ctx
            .lock()
            .map_err(|_| NuError::Disabled("Nu context poisoned".to_string()))?;
        ctx.eval_raw(script, input)
            .map_err(|e| NuError::Eval(e.to_string()))
    }

    pub fn complete(&self, spans: &[String]) -> Result<Vec<CompletionItem>, NuError> {
        let prefix = match spans.last() {
            Some(last) if !last.is_empty() => last,
            _ => return Ok(Vec::new()),
        };

        let escaped = prefix.replace('"', "\\\"");
        let script = format!(
            "let prefix = \"{}\"; help commands | get name | where {{ |name| $name | str starts-with $prefix }} | take 5",
            escaped
        );

        let data = self.run(&script, PipelineData::Empty)?;
        let mut items = Vec::new();
        for value in data.into_iter() {
            if let Value::String { val, .. } = value {
                items.push(CompletionItem { value: val });
            }
        }
        Ok(items)
    }

    pub fn render_output(&self, data: PipelineData) -> Result<String, NuError> {
        data.collect_string("\n", &self.config)
            .map_err(NuError::from)
    }
}

pub fn pipeline_from_selection(doc: &Rope, selection: &Selection) -> PipelineData {
    let values: Vec<Value> = selection
        .ranges()
        .iter()
        .map(|range| {
            let slice = doc.slice(range.from()..range.to()).to_string();
            Value::string(slice, Span::unknown())
        })
        .collect();

    match values.len() {
        0 => PipelineData::Value(Value::string(doc.slice(..).to_string(), Span::unknown()), None),
        1 => PipelineData::Value(values.into_iter().next().unwrap(), None),
        _ => PipelineData::Value(Value::list(values, Span::unknown()), None),
    }
}
