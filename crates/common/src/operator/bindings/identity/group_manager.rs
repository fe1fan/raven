use crate::runtime::bindings::{BindingMethod, BindingValue, NativeBinding};
use std::collections::HashMap;

pub struct GroupManagerBinding;

impl NativeBinding for GroupManagerBinding {
    fn name(&self) -> &'static str {
        "GroupManager"
    }

    fn methods(&self) -> Vec<BindingMethod> {
        vec![
            BindingMethod {
                name: "addGroup".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "deleteGroup".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "modifyGroup".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "getGroup".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "listGroups".to_string(),
                arity: 0,
                is_async: true,
            },
        ]
    }

    fn call(&self, method: &str, args: Vec<BindingValue>) -> BindingValue {
        match method {
            "addGroup" => self.add_group(args),
            "deleteGroup" => self.delete_group(args),
            "modifyGroup" => self.modify_group(args),
            "getGroup" => self.get_group(args),
            "listGroups" => self.list_groups(args),
            _ => BindingValue::Error(format!("Unknown method: {}", method)),
        }
    }
}

impl GroupManagerBinding {
    fn add_group(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("addGroup requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("addGroup params must be an object".to_string()),
        };

        let groupname = match params_obj.get("groupname") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("groupname is required".to_string()),
        };

        println!("[GroupManager] Adding group: {}", groupname);
        
        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert("groupname".to_string(), BindingValue::String(groupname));
        result.insert("message".to_string(), BindingValue::String("Group added successfully".to_string()));
        
        BindingValue::Object(result)
    }

    fn delete_group(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("deleteGroup requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("deleteGroup params must be an object".to_string()),
        };

        let groupname = match params_obj.get("groupname") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("groupname is required".to_string()),
        };

        println!("[GroupManager] Deleting group: {}", groupname);
        
        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert("message".to_string(), BindingValue::String("Group deleted successfully".to_string()));
        
        BindingValue::Object(result)
    }

    fn modify_group(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("modifyGroup requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("modifyGroup params must be an object".to_string()),
        };

        let groupname = match params_obj.get("groupname") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("groupname is required".to_string()),
        };

        println!("[GroupManager] Modifying group: {}", groupname);
        
        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert("message".to_string(), BindingValue::String("Group modified successfully".to_string()));
        
        BindingValue::Object(result)
    }

    fn get_group(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("getGroup requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("getGroup params must be an object".to_string()),
        };

        let groupname = match params_obj.get("groupname") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("groupname is required".to_string()),
        };

        println!("[GroupManager] Getting group: {}", groupname);
        
        let mut group_info = HashMap::new();
        group_info.insert("groupname".to_string(), BindingValue::String(groupname.clone()));
        group_info.insert("gid".to_string(), BindingValue::Int(2001));
        group_info.insert("members".to_string(), BindingValue::Array(vec![
            BindingValue::String("john".to_string()),
            BindingValue::String("jane".to_string()),
        ]));
        
        BindingValue::Object(group_info)
    }

    fn list_groups(&self, _args: Vec<BindingValue>) -> BindingValue {
        println!("[GroupManager] Listing groups");
        
        let groups = vec![
            {
                let mut group = HashMap::new();
                group.insert("groupname".to_string(), BindingValue::String("developers".to_string()));
                group.insert("gid".to_string(), BindingValue::Int(2001));
                BindingValue::Object(group)
            },
            {
                let mut group = HashMap::new();
                group.insert("groupname".to_string(), BindingValue::String("operators".to_string()));
                group.insert("gid".to_string(), BindingValue::Int(2002));
                BindingValue::Object(group)
            },
        ];
        
        BindingValue::Array(groups)
    }
}
