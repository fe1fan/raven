use crate::runtime::bindings::{BindingMethod, BindingValue, NativeBinding};
use std::collections::HashMap;

pub struct SudoManagerBinding;

impl NativeBinding for SudoManagerBinding {
    fn name(&self) -> &'static str {
        "SudoManager"
    }

    fn methods(&self) -> Vec<BindingMethod> {
        vec![
            BindingMethod {
                name: "addRule".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "removeRule".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "listRules".to_string(),
                arity: 0,
                is_async: true,
            },
        ]
    }

    fn call(&self, method: &str, args: Vec<BindingValue>) -> BindingValue {
        match method {
            "addRule" => self.add_rule(args),
            "removeRule" => self.remove_rule(args),
            "listRules" => self.list_rules(args),
            _ => BindingValue::Error(format!("Unknown method: {}", method)),
        }
    }
}

impl SudoManagerBinding {
    fn add_rule(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("addRule requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("addRule params must be an object".to_string()),
        };

        let user = params_obj.get("user")
            .and_then(|v| match v {
                BindingValue::String(s) => Some(s.clone()),
                _ => None,
            });

        let group = params_obj.get("group")
            .and_then(|v| match v {
                BindingValue::String(s) => Some(s.clone()),
                _ => None,
            });

        if user.is_none() && group.is_none() {
            return BindingValue::Error("Either user or group is required".to_string());
        }

        let target = user.unwrap_or_else(|| format!("%{}", group.unwrap()));

        println!("[SudoManager] Adding sudo rule for: {}", target);
        
        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert("message".to_string(), BindingValue::String("Sudo rule added successfully".to_string()));
        
        BindingValue::Object(result)
    }

    fn remove_rule(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("removeRule requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("removeRule params must be an object".to_string()),
        };

        let user = match params_obj.get("user") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("user is required".to_string()),
        };

        println!("[SudoManager] Removing sudo rule for: {}", user);
        
        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert("message".to_string(), BindingValue::String("Sudo rule removed successfully".to_string()));
        
        BindingValue::Object(result)
    }

    fn list_rules(&self, _args: Vec<BindingValue>) -> BindingValue {
        println!("[SudoManager] Listing sudo rules");
        
        // 返回模拟规则列表
        let rules = vec![
            {
                let mut rule = HashMap::new();
                rule.insert("user".to_string(), BindingValue::String("john".to_string()));
                rule.insert("hosts".to_string(), BindingValue::Array(vec![
                    BindingValue::String("ALL".to_string()),
                ]));
                rule.insert("commands".to_string(), BindingValue::Array(vec![
                    BindingValue::String("/usr/bin/systemctl".to_string()),
                ]));
                BindingValue::Object(rule)
            },
        ];
        
        BindingValue::Array(rules)
    }
}
