use crate::ast::RbatisAST;

use crate::string_util;
use rexpr;
use rexpr::ast::Node;
use rexpr::runtime::RExprRuntime;
use serde_json::Value;
use std::collections::LinkedList;

///the stmt replace str convert
pub trait StringConvert {
    fn convert(&self, index: usize) -> String;
}

///string抽象节点
#[derive(Clone, Debug)]
pub struct StringNode {
    pub value: String,
    //去重的，需要替换的要sql转换express map
    pub express_map: LinkedList<(String, String, Node)>,
}

impl StringNode {
    pub fn new(runtime: &RExprRuntime, v: &str) -> Result<Self, crate::error::Error> {
        let mut express_map = LinkedList::new();
        let list = string_util::find_convert_string(v);
        for (k, v) in list {
            let node = runtime.parse(&k)?;
            express_map.push_back((k, v, node));
        }
        Ok(Self {
            value: v.to_string(),
            express_map: express_map,
        })
    }
}

impl RbatisAST for StringNode {
    fn name() -> &'static str {
        "string"
    }
    fn eval(
        &self,
        convert: &dyn StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::error::Error> {
        let mut result = self.value.clone();
        for (item, value, node) in &self.express_map {
            if item.is_empty() {
                result = result.replace(value, "");
                continue;
            }
            if value.starts_with("#") {
                result = result.replace(value, convert.convert(arg_array.len()).as_str());
                let v = node.eval(env)?;
                arg_array.push(v);
            } else {
                let v = node.eval(env)?;
                if v.is_string() {
                    result = result.replace(value, &v.as_str().unwrap());
                } else {
                    result = result.replace(value, &v.to_string());
                }
            }
        }
        arg_sql.push_str(result.as_str());
        return Result::Ok(serde_json::Value::Null);
    }
}
