use crate::ast::RbatisAST;
use crate::error::Error;
use crate::node::node_type::NodeType;
use rexpr;
use rexpr::ast::Node;
use rexpr::runtime::RExprRuntime;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct BindNode {
    pub name: String,
    pub value: String,
    pub func: Node,
}

impl BindNode {
    pub fn def_name() -> &'static str {
        "let"
    }
    pub fn from(
        runtime: &RExprRuntime,
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::error::Error> {
        let source = source.trim();
        if express.starts_with(Self::def_name()) {
            let express = express[Self::def_name().len()..].trim();
            let name_value: Vec<&str> = express.split("=").collect();
            if name_value.len() != 2 {
                return Err(crate::error::Error::from(
                    "[rbatis] parser bind express fail:".to_string() + source,
                ));
            }
            return Ok(BindNode {
                name: name_value[0].to_owned(),
                value: name_value[1].to_owned(),
                func: runtime.parse(name_value[1])?,
            });
        } else if express.starts_with(Self::name()) {
            let express = express[Self::name().len()..].trim();
            let name_value: Vec<&str> = express.split("=").collect();
            if name_value.len() != 2 {
                return Err(crate::error::Error::from(
                    "[rbatis] parser bind express fail:".to_string() + source,
                ));
            }
            return Ok(BindNode {
                name: name_value[0].to_owned(),
                value: name_value[1].to_owned(),
                func: runtime.parse(name_value[1])?,
            });
        } else {
            return Err(Error::from(
                "[rbaits] OtherwiseNode must start with '_:' or 'otherwise:'",
            ));
        }
    }
}

impl RbatisAST for BindNode {
    fn name() -> &'static str {
        "bind"
    }
    fn eval(
        &self,
        convert: &dyn crate::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error> {
        let r = self.func.eval(env)?;
        env[self.name.as_str()] = r;
        return Result::Ok(serde_json::Value::Null);
    }
}
