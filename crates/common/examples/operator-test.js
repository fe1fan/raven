// 测试 Operator Identity 模块

import { UserManager } from 'raven/identity'
import { GroupManager } from 'raven/identity'
import { PermissionManager } from 'raven/identity'
import { SudoManager } from 'raven/identity'

// ========== 用户管理测试 ==========
console.log("========== 用户管理测试 ==========");

// 添加用户
await UserManager.addUser({
    username: 'john',
    password: 'hashed_password',
    uid: 1001,
    gid: 1001,
    home: '/home/john',
    shell: '/bin/bash',
    comment: 'John Doe',
    groups: ['developers', 'docker']
});

// 获取用户信息
var john = await UserManager.getUser({
    username: 'john'
});
console.log("用户信息:", JSON.stringify(john));

// 修改用户
await UserManager.modifyUser({
    username: 'john',
    shell: '/bin/zsh'
});

// 设置密码
await UserManager.setPassword({
    username: 'john',
    password: 'new_hashed_password'
});

// 列出用户
var users = await UserManager.listUsers({});
console.log("用户列表:", JSON.stringify(users));

// 锁定用户
await UserManager.lockUser({username: 'john'});

// 解锁用户
await UserManager.unlockUser({username: 'john'});

// 删除用户
await UserManager.deleteUser({
    username: 'john',
    removeHome: true
});

console.log("✓ 用户管理测试完成\n");

// ========== 组管理测试 ==========
console.log("========== 组管理测试 ==========");

// 添加组
await GroupManager.addGroup({
    groupname: 'developers',
    gid: 2001,
    members: ['john', 'jane']
});

// 获取组信息
var devGroup = await GroupManager.getGroup({
    groupname: 'developers'
});
console.log("组信息:", JSON.stringify(devGroup));

// 修改组
await GroupManager.modifyGroup({
    groupname: 'developers',
    newGroupname: 'dev-team',
    gid: 2002
});

// 列出组
var groups = await GroupManager.listGroups();
console.log("组列表:", JSON.stringify(groups));

// 删除组
await GroupManager.deleteGroup({
    groupname: 'dev-team'
});

console.log("✓ 组管理测试完成\n");

// ========== 权限管理测试 ==========
console.log("========== 权限管理测试 ==========");

// 设置文件权限
await PermissionManager.setFilePermission({
    path: '/var/www/html',
    mode: '755',
    recursive: true
});

// 设置文件所有者
await PermissionManager.setFileOwner({
    path: '/var/www/html',
    owner: 'www-data',
    group: 'www-data',
    recursive: true
});

// 设置 ACL
await PermissionManager.setACL({
    path: '/shared/project',
    entries: [
        {type: 'user', name: 'john', permissions: 'rwx'},
        {type: 'group', name: 'developers', permissions: 'rx'}
    ],
    recursive: true
});

// 获取 ACL
var acl = await PermissionManager.getACL({
    path: '/shared/project'
});
console.log("ACL信息:", JSON.stringify(acl));

// 设置 SELinux 上下文
await PermissionManager.setSELinuxContext({
    path: '/var/www/html',
    context: 'httpd_sys_content_t',
    recursive: true
});

console.log("✓ 权限管理测试完成\n");

// ========== Sudo 管理测试 ==========
console.log("========== Sudo 管理测试 ==========");

// 添加 sudo 规则
await SudoManager.addRule({
    user: 'john',
    hosts: ['ALL'],
    commands: ['/usr/bin/systemctl', '/usr/bin/docker'],
    runAs: 'root',
    noPassword: false
});

// 列出 sudo 规则
var rules = await SudoManager.listRules();
console.log("Sudo规则:", JSON.stringify(rules));

// 删除 sudo 规则
await SudoManager.removeRule({
    user: 'john',
    commands: ['/usr/bin/systemctl']
});

console.log("✓ Sudo管理测试完成\n");

console.log("========================================");
console.log("✅ 所有 Identity 模块测试完成！");
console.log("========================================");
