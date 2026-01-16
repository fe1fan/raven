
import {
    UserManager,
    GroupManager,
    PermissionManager,
    SudoManager
} from 'raven/identity'

// ---------- 用户管理 ----------
await UserManager.addUser({
    username: 'john',
    password: 'hashed_password',  // 已加密的密码
    uid: 1001,                     // 可选，不指定则自动分配
    gid: 1001,                     // 主组 ID
    home: '/home/john',            // 家目录
    shell: '/bin/bash',            // 默认 shell
    comment: 'John Doe',           // 用户描述
    groups: ['developers', 'docker'], // 附加组
    expireDate: '2025-12-31',      // 账户过期日期
    passwordExpireDays: 90         // 密码过期天数
})