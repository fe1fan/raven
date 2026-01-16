use crate::runtime::bindings::{BindingMethod, BindingValue, NativeBinding};
use std::collections::HashMap;

pub struct UserManagerBinding;

impl NativeBinding for UserManagerBinding {
    fn name(&self) -> &'static str {
        "UserManager"
    }

    fn methods(&self) -> Vec<BindingMethod> {
        vec![
            BindingMethod {
                name: "addUser".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "deleteUser".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "modifyUser".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "setPassword".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "getUser".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "listUsers".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "lockUser".to_string(),
                arity: 1,
                is_async: true,
            },
            BindingMethod {
                name: "unlockUser".to_string(),
                arity: 1,
                is_async: true,
            },
        ]
    }

    fn call(&self, method: &str, args: Vec<BindingValue>) -> BindingValue {
        match method {
            "addUser" => self.add_user(args),
            "deleteUser" => self.delete_user(args),
            "modifyUser" => self.modify_user(args),
            "setPassword" => self.set_password(args),
            "getUser" => self.get_user(args),
            "listUsers" => self.list_users(args),
            "lockUser" => self.lock_user(args),
            "unlockUser" => self.unlock_user(args),
            _ => BindingValue::Error(format!("Unknown method: {}", method)),
        }
    }
}

impl UserManagerBinding {
    fn add_user(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("addUser requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("addUser params must be an object".to_string()),
        };

        // 提取必需参数
        let username = match params_obj.get("username") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("username is required".to_string()),
        };

        let _password = match params_obj.get("password") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("password is required".to_string()),
        };

        // TODO: 实际的用户添加逻辑
        // 这里先返回模拟数据
        println!("[UserManager] Adding user: {}", username);

        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert("username".to_string(), BindingValue::String(username));
        result.insert(
            "message".to_string(),
            BindingValue::String("User added successfully".to_string()),
        );

        BindingValue::Object(result)
    }

    fn delete_user(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("deleteUser requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("deleteUser params must be an object".to_string()),
        };

        let username = match params_obj.get("username") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("username is required".to_string()),
        };

        // TODO: 实际的用户删除逻辑
        println!("[UserManager] Deleting user: {}", username);

        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert(
            "message".to_string(),
            BindingValue::String("User deleted successfully".to_string()),
        );

        BindingValue::Object(result)
    }

    fn modify_user(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("modifyUser requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("modifyUser params must be an object".to_string()),
        };

        let username = match params_obj.get("username") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("username is required".to_string()),
        };

        // TODO: 实际的用户修改逻辑
        println!("[UserManager] Modifying user: {}", username);

        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert(
            "message".to_string(),
            BindingValue::String("User modified successfully".to_string()),
        );

        BindingValue::Object(result)
    }

    fn set_password(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("setPassword requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("setPassword params must be an object".to_string()),
        };

        let username = match params_obj.get("username") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("username is required".to_string()),
        };

        // TODO: 实际的密码设置逻辑
        println!("[UserManager] Setting password for user: {}", username);

        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert(
            "message".to_string(),
            BindingValue::String("Password set successfully".to_string()),
        );

        BindingValue::Object(result)
    }

    fn get_user(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("getUser requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("getUser params must be an object".to_string()),
        };

        let username = match params_obj.get("username") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("username is required".to_string()),
        };

        // TODO: 实际的用户查询逻辑
        println!("[UserManager] Getting user: {}", username);

        let mut user_info = HashMap::new();
        user_info.insert(
            "username".to_string(),
            BindingValue::String(username.clone()),
        );
        user_info.insert("uid".to_string(), BindingValue::Int(1001));
        user_info.insert("gid".to_string(), BindingValue::Int(1001));
        user_info.insert(
            "home".to_string(),
            BindingValue::String(format!("/home/{}", username)),
        );
        user_info.insert(
            "shell".to_string(),
            BindingValue::String("/bin/bash".to_string()),
        );

        BindingValue::Object(user_info)
    }

    fn list_users(&self, args: Vec<BindingValue>) -> BindingValue {
        // 可选参数
        let _params = args.get(0);

        // TODO: 实际的用户列表逻辑
        println!("[UserManager] Listing users");

        // 返回模拟用户列表
        let users = vec![
            {
                let mut user = HashMap::new();
                user.insert(
                    "username".to_string(),
                    BindingValue::String("john".to_string()),
                );
                user.insert("uid".to_string(), BindingValue::Int(1001));
                BindingValue::Object(user)
            },
            {
                let mut user = HashMap::new();
                user.insert(
                    "username".to_string(),
                    BindingValue::String("jane".to_string()),
                );
                user.insert("uid".to_string(), BindingValue::Int(1002));
                BindingValue::Object(user)
            },
        ];

        BindingValue::Array(users)
    }

    fn lock_user(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("lockUser requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("lockUser params must be an object".to_string()),
        };

        let username = match params_obj.get("username") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("username is required".to_string()),
        };

        // TODO: 实际的用户锁定逻辑
        println!("[UserManager] Locking user: {}", username);

        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert(
            "message".to_string(),
            BindingValue::String("User locked successfully".to_string()),
        );

        BindingValue::Object(result)
    }

    fn unlock_user(&self, args: Vec<BindingValue>) -> BindingValue {
        let params = match args.get(0) {
            Some(p) => p,
            None => return BindingValue::Error("unlockUser requires params object".to_string()),
        };

        let params_obj = match params {
            BindingValue::Object(obj) => obj,
            _ => return BindingValue::Error("unlockUser params must be an object".to_string()),
        };

        let username = match params_obj.get("username") {
            Some(BindingValue::String(s)) => s.clone(),
            _ => return BindingValue::Error("username is required".to_string()),
        };

        // TODO: 实际的用户解锁逻辑
        println!("[UserManager] Unlocking user: {}", username);

        let mut result = HashMap::new();
        result.insert("success".to_string(), BindingValue::Bool(true));
        result.insert(
            "message".to_string(),
            BindingValue::String("User unlocked successfully".to_string()),
        );

        BindingValue::Object(result)
    }
}
