use py_sql::py_sql::PyRuntime;
use py_sql::{RExprRuntime, StringConvert};
use rexpr::bencher::QPS;

pub struct DriverType {}

impl StringConvert for DriverType {
    fn convert(&self, index: usize) -> String {
        "?".to_string()
    }
}

//cargo test --release --package py_sql --test bench_test test_bench_py_sqsl_select --no-fail-fast -- --exact -Z unstable-options --show-output
#[test]
pub fn test_bench_py_sqsl_select() {
    let py_runtime = PyRuntime::new(vec![]);
    let engine = RExprRuntime::new();
    let (sql,arg) = py_runtime.eval(&DriverType{}, "select * from table where
                                                                  id = #{id}
                                                                  id != #{id}
                                                                  id in #{ids}
                                                                  id in #{ids}
                                                                  id in #{ids}
                                                                  id not in #{ids}
                                                                  for k,v in map:
                                                                     #{k}=#{v}
                                                                  name like #{name}
                                                                  or
                                                                  name not like #{name}
                                                                  create_time between #{create_time} and #{create_time}
                                                                  group by
                                                                  for item in ids:
                                                                     #{item}
                                                                  order by
                                                                  for item in order_by:
                                                                     #{item}",
                                    &mut serde_json::json!({"id":1,"order_by":["id","name"],"ids":[1,2,3],"name":"asdf","map":{"a":1},"create_time":"2020-23-23"}), &engine).unwrap();
    println!("sql:{},arg:{:?}", sql, arg);
    let total = 10000;
    let now = std::time::Instant::now();
    for _ in 0..total {
        py_runtime.eval(&DriverType{}, "select * from table where
                                                                  id = #{id}
                                                                  id != #{id}
                                                                  id in #{ids}
                                                                  id in #{ids}
                                                                  id in #{ids}
                                                                  id not in #{ids}
                                                                  for k,v in map:
                                                                     #{k}=#{v}
                                                                  name like #{name}
                                                                  or
                                                                  name not like #{name}
                                                                  create_time between #{create_time} and #{create_time}
                                                                  group by
                                                                  for item in ids:
                                                                     #{item}
                                                                  order by
                                                                  for item in order_by:
                                                                     #{item}",
                        &mut serde_json::json!({"id":1,"order_by":["id","name"],"ids":[1,2,3],"name":"asdf","map":{"a":1},"create_time":"2020-23-23"}), &engine).unwrap();
    }
    now.time(total);
    now.qps(total);
}
