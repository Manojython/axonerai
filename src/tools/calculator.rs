use crate::tool::Tool;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct Calculator;

#[derive(Debug, Deserialize, Serialize)]
struct CalculatorInput {
    operation: String,
    a: f64,
    b: f64,
}
#[async_trait]
impl Tool for Calculator {
    fn name(&self) -> String {
        "calculator".to_string()
    }

    fn description(&self) -> String {
        "Perform basic arithmetic operations (add, subtract, multiply, divide)".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["add", "subtract", "multiply", "divide"],
                    "description": "The arithmetic operation to perform"
                },
                "a": {
                    "type": "number",
                    "description": "First number"
                },
                "b": {
                    "type": "number",
                    "description": "Second number"
                }
            },
            "required": ["operation", "a", "b"]
        })
    }

   async fn execute(&self, input: Value) -> Result<String> {

        let input: CalculatorInput = serde_json::from_value(input)
            .map_err(|e| anyhow!("Invalid calculator input: {}", e))?;

        let result = match input.operation.as_str() {
            "add" => input.a + input.b,
            "subtract" => input.a - input.b,
            "multiply" => input.a * input.b,
            "divide" => {
                if input.b == 0.0 {
                    return Err(anyhow!("Cannot divide by zero"));
                }
                input.a / input.b
            }
            _ => return Err(anyhow!("Unknown operation: {}", input.operation)),
        };

        Ok(result.to_string())
    }
}