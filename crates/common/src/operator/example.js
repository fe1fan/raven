/**
 * Raven - 运维 API 完整文档
 *
 * 这是一个完整的运维操作 API 示例文档
 * 所有 API 都返回 Promise，支持 async/await
 */

// ============================================
// 用户与权限管理体系
// ============================================

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

await UserManager.deleteUser({
    username: 'john',
    removeHome: true,// 是否删除家目录
    force: false                   // 强制删除（即使用户已登录）
})

await UserManager.modifyUser({
    username: 'john',
    newUsername: 'john_doe',       // 修改用户名
    uid: 1002,                     // 修改 UID
    gid: 1002,                     // 修改主组
    home: '/home/john_doe',        // 修改家目录
    shell: '/bin/zsh',             // 修改 shell
    comment: 'John Doe - Senior Dev',
    lock: false,                   // 锁定/解锁账户
    expireDate: '2026-12-31'
})

await UserManager.setPassword({
    username: 'john',
    password: 'new_hashed_password',
    forceChange: true              // 下次登录强制修改密码
})

const user = await UserManager.getUser({
    username: 'john'
})
// 返回: { username, uid, gid, home, shell, groups, lastLogin, ... }

const users = await UserManager.listUsers({
    filter: {
        uidMin: 1000,              // UID 范围过滤
        uidMax: 2000,
        groups: ['developers']      // 按组过滤
    }
})

await UserManager.lockUser({username: 'john'})
await UserManager.unlockUser({username: 'john'})

// ---------- 组管理 ----------
await GroupManager.addGroup({
    groupname: 'developers',
    gid: 2001,                     // 可选
    members: ['john', 'jane']      // 初始成员
})

await GroupManager.deleteGroup({
    groupname: 'developers',
    force: false
})

await GroupManager.modifyGroup({
    groupname: 'developers',
    newGroupname: 'dev-team',
    gid: 2002,
    addMembers: ['bob'],
    removeMembers: ['alice']
})

const group = await GroupManager.getGroup({
    groupname: 'developers'
})

const groups = await GroupManager.listGroups()

// ---------- 权限管理 ----------
await PermissionManager.setFilePermission({
    path: '/var/www/html',
    mode: '755',                   // 或 0o755
    recursive: true
})

await PermissionManager.setFileOwner({
    path: '/var/www/html',
    owner: 'www-data',
    group: 'www-data',
    recursive: true
})

await PermissionManager.setACL({
    path: '/shared/project',
    entries: [
        {type: 'user', name: 'john', permissions: 'rwx'},
        {type: 'group', name: 'developers', permissions: 'rx'},
        {type: 'other', permissions: '---'}
    ],
    recursive: true,
    default: true                  // 设置默认 ACL
})

const acl = await PermissionManager.getACL({
    path: '/shared/project'
})

await PermissionManager.setSELinuxContext({
    path: '/var/www/html',
    context: 'httpd_sys_content_t',
    recursive: true
})

// ---------- Sudo 管理 ----------
await SudoManager.addRule({
    user: 'john',                  // 或 group: 'developers'
    hosts: ['ALL'],
    commands: ['/usr/bin/systemctl', '/usr/bin/docker'],
    runAs: 'root',
    noPassword: false,
    tags: ['NOPASSWD', 'SETENV']
})

await SudoManager.removeRule({
    user: 'john',
    commands: ['/usr/bin/systemctl']
})

const rules = await SudoManager.listRules()


// ============================================
// 文件系统管理
// ============================================

import {
    FileManager,
    DirectoryManager,
    DiskManager,
    MountManager
} from 'raven/filesystem'

// ---------- 文件操作 ----------
await FileManager.create({
    path: '/tmp/test.txt',
    content: 'Hello World',        // 或 buffer
    mode: '644',
    owner: 'root',
    group: 'root',
    encoding: 'utf-8'
})

await FileManager.write({
    path: '/tmp/test.txt',
    content: 'New content',
    append: false,                 // true 为追加模式
    encoding: 'utf-8'
})

const content = await FileManager.read({
    path: '/tmp/test.txt',
    encoding: 'utf-8'              // 或 null 返回 Buffer
})

await FileManager.copy({
    source: '/tmp/test.txt',
    destination: '/tmp/test_backup.txt',
    preserveAttributes: true,      // 保留权限、时间戳等
    overwrite: false
})

await FileManager.move({
    source: '/tmp/test.txt',
    destination: '/var/log/test.txt'
})

await FileManager.delete({
    path: '/tmp/test.txt',
    force: false
})

const stat = await FileManager.stat({
    path: '/tmp/test.txt'
})
// 返回: { size, mode, owner, group, atime, mtime, ctime, ... }

const exists = await FileManager.exists({
    path: '/tmp/test.txt'
})

// ---------- 目录操作 ----------
await DirectoryManager.create({
    path: '/var/app/data',
    mode: '755',
    recursive: true,               // 创建父目录
    owner: 'app',
    group: 'app'
})

await DirectoryManager.delete({
    path: '/var/app/data',
    recursive: true,               // 递归删除
    force: false
})

const files = await DirectoryManager.list({
    path: '/var/log',
    recursive: false,
    filter: {
        pattern: '*.log',          // glob 模式
        type: 'file',              // 'file' | 'directory' | 'symlink'
        minSize: 1024,             // 字节
        maxSize: 1048576,
        modifiedAfter: '2024-01-01',
        modifiedBefore: '2024-12-31'
    },
    sort: 'mtime',                 // 'name' | 'size' | 'mtime'
    order: 'desc'
})

const size = await DirectoryManager.getSize({
    path: '/var/log',
    recursive: true
})

// ---------- 磁盘管理 ----------
const disks = await DiskManager.listDisks()
// 返回: [{ device: '/dev/sda', size, type, model, ... }]

const partitions = await DiskManager.listPartitions({
    device: '/dev/sda'
})

await DiskManager.createPartition({
    device: '/dev/sda',
    size: '100G',                  // 或 '100%' 使用剩余空间
    type: 'ext4',
    label: 'data'
})

await DiskManager.deletePartition({
    device: '/dev/sda1',
    force: false
})

await DiskManager.formatPartition({
    device: '/dev/sda1',
    filesystem: 'ext4',            // ext4, xfs, btrfs, ntfs
    label: 'data',
    options: ['-F']                // 文件系统特定选项
})

const usage = await DiskManager.getUsage({
    path: '/'                      // 或 device: '/dev/sda1'
})
// 返回: { total, used, available, percentage }

// ---------- 挂载管理 ----------
await MountManager.mount({
    device: '/dev/sda1',
    mountPoint: '/mnt/data',
    filesystem: 'ext4',
    options: ['rw', 'noatime'],
    persistent: true               // 写入 /etc/fstab
})

await MountManager.unmount({
    mountPoint: '/mnt/data',
    force: false,
    lazy: false                    // lazy unmount
})

const mounts = await MountManager.listMounts()

await MountManager.remount({
    mountPoint: '/mnt/data',
    options: ['ro']                // 重新挂载为只读
})


// ============================================
// 进程与服务管理
// ============================================

import {
    ProcessManager,
    ServiceManager,
    CronManager,
    SystemdManager
} from 'raven/process'

// ---------- 进程管理 ----------
const processes = await ProcessManager.list({
    filter: {
        user: 'www-data',
        name: 'nginx',
        cpu: {min: 50},          // CPU 使用率 > 50%
        memory: {min: 100 * 1024 * 1024} // 内存 > 100MB
    },
    sort: 'cpu',
    order: 'desc'
})

const process = await ProcessManager.get({
    pid: 1234
})
// 返回: { pid, name, user, cpu, memory, status, startTime, ... }

await ProcessManager.kill({
    pid: 1234,
    signal: 'SIGTERM'              // SIGTERM, SIGKILL, SIGHUP, etc.
})

await ProcessManager.killByName({
    name: 'nginx',
    signal: 'SIGTERM',
    user: 'www-data'               // 可选，只杀特定用户的进程
})

const stats = await ProcessManager.getSystemStats()
// 返回: { cpuUsage, memoryUsage, loadAverage, uptime, ... }

// ---------- 服务管理 (通用接口) ----------
await ServiceManager.start({
    service: 'nginx'
})

await ServiceManager.stop({
    service: 'nginx',
    force: false
})

await ServiceManager.restart({
    service: 'nginx'
})

await ServiceManager.reload({
    service: 'nginx'
})

await ServiceManager.enable({
    service: 'nginx'               // 开机自启
})

await ServiceManager.disable({
    service: 'nginx'
})

const status = await ServiceManager.getStatus({
    service: 'nginx'
})
// 返回: { active, enabled, pid, uptime, memory, ... }

const services = await ServiceManager.list({
    filter: {
        active: true,
        enabled: true
    }
})

// ---------- Systemd 管理 ----------
await SystemdManager.createUnit({
    name: 'myapp',
    type: 'service',               // service, timer, socket, etc.
    config: {
        Unit: {
            Description: 'My Application',
            After: 'network.target'
        },
        Service: {
            Type: 'simple',
            User: 'app',
            ExecStart: '/usr/bin/myapp',
            Restart: 'always',
            RestartSec: '10s'
        },
        Install: {
            WantedBy: 'multi-user.target'
        }
    }
})

await SystemdManager.deleteUnit({
    name: 'myapp',
    type: 'service'
})

await SystemdManager.editUnit({
    name: 'myapp',
    type: 'service',
    config: { /* 部分配置 */}
})

await SystemdManager.daemonReload()

const logs = await SystemdManager.getLogs({
    unit: 'nginx',
    lines: 100,
    follow: false,
    since: '2024-01-01',
    until: '2024-12-31',
    priority: 'err'                // emerg, alert, crit, err, warning, notice, info, debug
})

// ---------- 定时任务管理 ----------
await CronManager.addJob({
    user: 'root',                  // 可选，默认 root
    schedule: '0 2 * * *',         // cron 表达式
    command: '/usr/local/bin/backup.sh',
    comment: 'Daily backup',
    environment: {
        PATH: '/usr/local/bin:/usr/bin:/bin'
    }
})

await CronManager.removeJob({
    user: 'root',
    id: 'job-id'                   // 或 command: '/usr/local/bin/backup.sh'
})

const jobs = await CronManager.listJobs({
    user: 'root'
})

await CronManager.enableJob({
    user: 'root',
    id: 'job-id'
})

await CronManager.disableJob({
    user: 'root',
    id: 'job-id'
})


// ============================================
// 网络管理
// ============================================

import {
    NetworkManager,
    FirewallManager,
    DNSManager,
    RouteManager
} from 'raven/network'

// ---------- 网络接口管理 ----------
const interfaces = await NetworkManager.listInterfaces()

const iface = await NetworkManager.getInterface({
    name: 'eth0'
})
// 返回: { name, mac, ipv4, ipv6, status, speed, mtu, ... }

await NetworkManager.setInterface({
    name: 'eth0',
    ipv4: '192.168.1.100/24',
    gateway: '192.168.1.1',
    dns: ['8.8.8.8', '8.8.4.4'],
    mtu: 1500,
    up: true
})

await NetworkManager.addIPAddress({
    interface: 'eth0',
    address: '192.168.1.101/24'
})

await NetworkManager.removeIPAddress({
    interface: 'eth0',
    address: '192.168.1.101/24'
})

await NetworkManager.upInterface({
    name: 'eth0'
})

await NetworkManager.downInterface({
    name: 'eth0'
})

const stats = await NetworkManager.getStats({
    interface: 'eth0'
})
// 返回: { rxBytes, txBytes, rxPackets, txPackets, errors, ... }

// ---------- 防火墙管理 ----------
await FirewallManager.addRule({
    chain: 'INPUT',                // INPUT, OUTPUT, FORWARD
    protocol: 'tcp',               // tcp, udp, icmp, all
    source: '0.0.0.0/0',
    destination: '192.168.1.100',
    destinationPort: 80,
    action: 'ACCEPT',// ACCEPT, DROP, REJECT
    comment: 'Allow HTTP'
})

await FirewallManager.removeRule({
    chain: 'INPUT',
    ruleNumber: 5  // 或使用完整规则匹配
})

const rules = await FirewallManager.listRules({
    chain: 'INPUT'
})

await FirewallManager.setDefaultPolicy({
    chain: 'INPUT',
    policy: 'DROP'                 // ACCEPT, DROP
})

await FirewallManager.flush({
    chain: 'INPUT'                 // 清空规则
})

await FirewallManager.save()       // 保存规则（持久化）

await FirewallManager.restore()    // 恢复规则

// NAT 规则
await FirewallManager.addNATRule({
    type: 'SNAT',                  // SNAT, DNAT, MASQUERADE
    source: '192.168.1.0/24',
    destination: '0.0.0.0/0',
    toSource: '203.0.113.1'
})

// 端口转发
await FirewallManager.addPortForward({
    protocol: 'tcp',
    externalPort: 8080,
    internalIP: '192.168.1.100',
    internalPort: 80
})

// ---------- DNS 管理 ----------
await DNSManager.setServers({
    servers: ['8.8.8.8', '8.8.4.4', '1.1.1.1']
})

const servers = await DNSManager.getServers()

await DNSManager.addHost({
    hostname: 'myapp.local',
    ip: '192.168.1.100'
})

await DNSManager.removeHost({
    hostname: 'myapp.local'
})

const hosts = await DNSManager.listHosts()

const result = await DNSManager.lookup({
    hostname: 'example.com',
    type: 'A'      // A, AAAA, MX, TXT, etc.
})

// ---------- 路由管理 ----------
await RouteManager.addRoute({
    destination: '10.0.0.0/8',
    gateway: '192.168.1.1',
    interface: 'eth0',
    metric: 100
})

await RouteManager.removeRoute({
    destination: '10.0.0.0/8'
})

const routes = await RouteManager.listRoutes()

await RouteManager.setDefaultGateway({
    gateway: '192.168.1.1',
    interface: 'eth0'
})


// ============================================
// 软件包管理
// ============================================

import {
    PackageManager,
    RepositoryManager,
    UpdateManager
} from 'raven/package'

// ---------- 软件包管理 ----------
await PackageManager.install({
    packages: ['nginx', 'postgresql'],
    version: 'latest',             // 或指定版本 '1.20.1'
    yes: true,                     // 自动确认
    noDeps: false                  // 不安装依赖
})

await PackageManager.uninstall({
    packages: ['nginx'],
    purge: true,                   // 删除配置文件
    autoremove: true               // 删除不需要的依赖
})

await PackageManager.update({
    packages: ['nginx']// 不指定则更新所有
})

const info = await PackageManager.getInfo({
    package: 'nginx'
})
// 返回: { name, version, description, dependencies, size, ... }

const installed = await PackageManager.listInstalled({
    filter: {
        name: 'nginx*',            // 支持通配符
        upgradable: true
    }
})

const available = await PackageManager.search({
    query: 'web server',
    limit: 50
})

const upgradable = await PackageManager.listUpgradable()

// ---------- 仓库管理 ----------
await RepositoryManager.add({
    name: 'nginx-stable',
    url: 'http://nginx.org/packages/ubuntu',
    distribution: 'focal',
    components: ['nginx'],
    gpgKey: 'https://nginx.org/keys/nginx_signing.key'
})

await RepositoryManager.remove({
    name: 'nginx-stable'
})

const repos = await RepositoryManager.list()

await RepositoryManager.enable({
    name: 'nginx-stable'
})

await RepositoryManager.disable({
    name: 'nginx-stable'
})

await RepositoryManager.refresh()  // 更新软件包列表

// ---------- 系统更新管理 ----------
await UpdateManager.checkUpdates()

const updates = await UpdateManager.listUpdates({
    security: true                 // 只列出安全更新
})

await UpdateManager.upgrade({
    security: true,                // 只安装安全更新
    yes: true,
    reboot: false  // 更新后重启
})

await UpdateManager.distUpgrade({
    yes: true
})

const history = await UpdateManager.getHistory({
    limit: 100
})


// ============================================
// 容器管理
// ============================================

import {
    DockerManager,
    ContainerManager,
    ImageManager,
    VolumeManager,
    NetworkManager as DockerNetworkManager
} from 'raven/container'

// ---------- 容器管理 ----------
await ContainerManager.create({
    name: 'myapp',
    image: 'nginx:latest',
    ports: [
        {host: 8080, container: 80}
    ],
    volumes: [
        {host: '/data', container: '/usr/share/nginx/html'}
    ],
    environment: {
        ENV: 'production'
    },
    restart: 'always',             // no, always, on-failure, unless-stopped
    network: 'bridge',
    command: ['nginx', '-g', 'daemon off;'],
    labels: {
        'app': 'myapp',
        'version': '1.0'
    },
    resources: {
        cpus: '2',
        memory: '1g'
    }
})

await ContainerManager.start({
    container: 'myapp'
})

await ContainerManager.stop({
    container: 'myapp',
    timeout: 10                    // 秒
})

await ContainerManager.restart({
    container: 'myapp'
})

await ContainerManager.remove({
    container: 'myapp',
    force: false,
    volumes: true                  // 同时删除关联的卷
})

const containers = await ContainerManager.list({
    all: true,                     // 包括停止的容器
    filter: {
        status: 'running',         // running, exited, paused
        label: 'app=myapp'
    }
})

const container = await ContainerManager.inspect({
    container: 'myapp'
})

const logs = await ContainerManager.logs({
    container: 'myapp',
    follow: false,
    tail: 100,
    since: '2024-01-01',
    timestamps: true
})

const stats = await ContainerManager.stats({
    container: 'myapp'
})
// 返回: { cpu, memory, network, blockIO, ... }

await ContainerManager.exec({
    container: 'myapp',
    command: ['ls', '-la'],
    user: 'root',
    workdir: '/app',
    tty: true,
    interactive: true
})

// ---------- 镜像管理 ----------
await ImageManager.pull({
    image: 'nginx:latest',
    registry: 'docker.io'          // 可选
})

await ImageManager.build({
    context: '/path/to/dockerfile',
    tag: 'myapp:1.0',
    dockerfile: 'Dockerfile',
    buildArgs: {
        VERSION: '1.0'
    },
    noCache: false
})

await ImageManager.push({
    image: 'myapp:1.0',
    registry: 'registry.example.com'
})

await ImageManager.remove({
    image: 'myapp:1.0',
    force: false
})

const images = await ImageManager.list({
    filter: {
        dangling: false,           // 悬空镜像
        label: 'app=myapp'
    }
})

const image = await ImageManager.inspect({
    image: 'nginx:latest'
})

await ImageManager.tag({
    source: 'myapp:1.0',
    target: 'myapp:latest'
})

await ImageManager.prune({
    all: false,                    // 删除所有未使用的镜像
    filter: {
        until: '24h'               // 删除 24 小时前的镜像
    }
})

// ---------- 卷管理 ----------
await VolumeManager.create({
    name: 'mydata',
    driver: 'local',
    labels: {
        'app': 'myapp'
    }
})

await VolumeManager.remove({
    volume: 'mydata',
    force: false
})

const volumes = await VolumeManager.list({
    filter: {
        dangling: false,
        label: 'app=myapp'
    }
})

const volume = await VolumeManager.inspect({
    volume: 'mydata'
})

await VolumeManager.prune()

// ---------- Docker 网络管理 ----------
await DockerNetworkManager.create({
    name: 'mynetwork',
    driver: 'bridge',              // bridge, host, overlay, macvlan
    subnet: '172.20.0.0/16',
    gateway: '172.20.0.1',
    ipRange: '172.20.10.0/24',
    labels: {
        'app': 'myapp'
    }
})

await DockerNetworkManager.remove({
    network: 'mynetwork'
})

await DockerNetworkManager.connect({
    network: 'mynetwork',
    container: 'myapp',
    ipv4Address: '172.20.10.10'
})

await DockerNetworkManager.disconnect({
    network: 'mynetwork',
    container: 'myapp',
    force: false
})

const networks = await DockerNetworkManager.list()

const network = await DockerNetworkManager.inspect({
    network: 'mynetwork'
})


// ============================================
// 监控与日志
// ============================================

import {
    MonitorManager,
    LogManager,
    AlertManager,
    MetricsManager
} from 'raven/monitor'

// ---------- 系统监控 ----------
const metrics = await MonitorManager.getSystemMetrics({
    interval: '1m',                // 采样间隔
    duration: '1h'                 // 时间范围
})
// 返回: { cpu, memory, disk, network, ... }

const cpuUsage = await MonitorManager.getCPUUsage({
    perCore: true
})

const memoryUsage = await MonitorManager.getMemoryUsage()

const diskIO = await MonitorManager.getDiskIO({
    device: '/dev/sda'
})

const networkIO = await MonitorManager.getNetworkIO({
    interface: 'eth0'
})

const loadAverage = await MonitorManager.getLoadAverage()

// ---------- 日志管理 ----------
const logs = await LogManager.query({
    source: '/var/log/nginx/access.log',
    filter: {
        level: 'error',// debug, info, warning, error, critical
        pattern: '404',            // 正则表达式
        since: '2024-01-01',
        until: '2024-12-31'
    },
    limit: 1000,
    sort: 'desc'
})

await LogManager.rotate({
    file: '/var/log/nginx/access.log',
    maxSize: '100M',
    maxAge: '30d',
    compress: true
})

await LogManager.archive({
    source: '/var/log/nginx',
    destination: '/backup/logs',
    compress: true,
    delete: false                  // 归档后删除原文件
})

const logFiles = await LogManager.list({
    directory: '/var/log',
    pattern: '*.log',
    recursive: true
})

// ---------- 告警管理 ----------
await AlertManager.createRule({
    name: 'high-cpu-usage',
    condition: {
        metric: 'cpu.usage',
        operator: '>',
        threshold: 80,
        duration: '5m'             // 持续时间
    },
    actions: [
        {
            type: 'email',
            to: ['admin@example.com'],
            subject: 'High CPU Usage Alert'
        },
        {
            type: 'webhook',
            url: 'https://hooks.slack.com/...'
        }
    ],
    enabled: true
})

await AlertManager.deleteRule({
    name: 'high-cpu-usage'
})

const rules = await AlertManager.listRules()

const alerts = await AlertManager.listAlerts({
    status: 'firing',              // firing, resolved
    since: '2024-01-01'
})

await AlertManager.acknowledgeAlert({
    id: 'alert-id',
    comment: 'Investigating'
})

// ---------- 指标管理 ----------
await MetricsManager.record({
    name: 'http.requests',
    value: 1,
    labels: {
        method: 'GET',
        status: '200'
    },
    timestamp: Date.now()
})

const metrics = await MetricsManager.query({
    name: 'http.requests',
    labels: {
        method: 'GET'
    },
    start: '2024-01-01',
    end: '2024-12-31',
    step: '1m'                     // 聚合间隔
})

await MetricsManager.createDashboard({
    name: 'System Overview',
    panels: [
        {
            title: 'CPU Usage',
            metric: 'cpu.usage',
            type: 'line'           // line, bar, gauge, table
        },
        {
            title: 'Memory Usage',
            metric: 'memory.usage',
            type: 'gauge'
        }
    ]
})


// ============================================
// 备份与恢复
// ============================================

import {
    BackupManager,
    SnapshotManager,
    RestoreManager
} from 'raven/backup'

// ---------- 备份管理 ----------
await BackupManager.create({
    name: 'daily-backup',
    sources: [
        '/var/www',
        '/etc/nginx',
        '/home'
    ],
    destination: '/backup',
    compression: 'gzip',           // gzip, bzip2, xz, none
    encryption: {
        enabled: true,
        password: 'secret'
    },
    exclude: [
        '*.log',
        '*.tmp',
        'node_modules'
    ],
    incremental: false,            // 增量备份
    schedule: '0 2 * * *'          // cron 表达式
})

await BackupManager.run({
    name: 'daily-backup'
})

await BackupManager.delete({
    name: 'daily-backup',
    keepLast: 7                    // 保留最近 7 个备份
})

const backups = await BackupManager.list({
    filter: {
        name: 'daily-backup',
        since: '2024-01-01'
    }
})

const backup = await BackupManager.getInfo({
    id: 'backup-id'
})
// 返回: { id, name, size, created, sources, status, ... }

await BackupManager.verify({
    id: 'backup-id'                // 验证备份完整性
})

await BackupManager.schedule({
    name: 'daily-backup',
    cron: '0 2 * * *',
    retention: {
        daily: 7,                  // 保留 7 天
        weekly: 4,                 // 保留 4 周
        monthly: 12                // 保留 12 月
    }
})

// ---------- 快照管理 ----------
await SnapshotManager.create({
    name: 'before-upgrade',
    volume: '/dev/vg0/lv_root',    // LVM 卷
    size: '10G',                   // 快照大小
    description: 'Before system upgrade'
})

await SnapshotManager.delete({
    name: 'before-upgrade'
})

const snapshots = await SnapshotManager.list({
    volume: '/dev/vg0/lv_root'
})

await SnapshotManager.merge({
    snapshot: 'before-upgrade'     // 合并快照到原卷
})

// Docker 卷快照
await SnapshotManager.createVolumeSnapshot({
    volume: 'mydata',
    name: 'mydata-snapshot'
})

// 文件系统快照 (Btrfs/ZFS)
await SnapshotManager.createFilesystemSnapshot({
    path: '/data',
    name: 'data-snapshot',
    readonly: true
})

// ---------- 恢复管理 ----------
await RestoreManager.restore({
    backupId: 'backup-id',
    destination: '/restore',
    overwrite: false,
    preservePermissions: true,
    files: [                       // 可选，只恢复特定文件
        '/var/www/index.html',
        '/etc/nginx/nginx.conf'
    ]
})

await RestoreManager.restoreFromSnapshot({
    snapshot: 'before-upgrade',
    destination: '/restore'
})

const restores = await RestoreManager.listRestores({
    status: 'completed'            // pending, running, completed, failed
})

await RestoreManager.cancel({
    id: 'restore-id'
})


// ============================================
// 安全管理
// ============================================

import {
    SecurityManager,
    CertificateManager,
    AuditManager,
    ComplianceManager
} from 'raven/security'

// ---------- 安全扫描 ----------
const vulnerabilities = await SecurityManager.scanVulnerabilities({
    target: 'system',              // system, packages, containers
    severity: ['high', 'critical']
})

await SecurityManager.scanPorts({
    host: 'localhost',
    ports: 'all'                   // 或 [80, 443, 22]
})

const malware = await SecurityManager.scanMalware({
    path: '/var/www',
    recursive: true,
    quarantine: true               // 隔离恶意文件
})

await SecurityManager.updateDefinitions()  // 更新病毒库

// ---------- 证书管理 ----------
await CertificateManager.generate({
    type: 'self-signed',           // self-signed, csr, letsencrypt
    commonName: 'example.com',
    organization: 'Example Inc',
    country: 'US',
    validDays: 365,
    keySize: 2048,
    output: {
        cert: '/etc/ssl/certs/example.crt',
        key: '/etc/ssl/private/example.key'
    }
})

await CertificateManager.generateCSR({
    commonName: 'example.com',
    organization: 'Example Inc',
    country: 'US',
    output: '/etc/ssl/csr/example.csr'
})

await CertificateManager.install({
    cert: '/path/to/cert.crt',
    key: '/path/to/key.key',
    chain: '/path/to/chain.crt',   // 可选
    destination: '/etc/ssl'
})

const certs = await CertificateManager.list({
    path: '/etc/ssl/certs'
})

const cert = await CertificateManager.inspect({
    path: '/etc/ssl/certs/example.crt'
})
// 返回: { subject, issuer, validFrom, validTo, fingerprint, ... }

await CertificateManager.verify({
    cert: '/etc/ssl/certs/example.crt',
    chain: '/etc/ssl/certs/chain.crt'
})

const expiring = await CertificateManager.checkExpiration({
    path: '/etc/ssl/certs',
    days: 30                       // 30 天内过期的证书
})

// Let's Encrypt
await CertificateManager.obtainLetsEncrypt({
    domains: ['example.com', 'www.example.com'],
    email: 'admin@example.com',
    webroot: '/var/www/html',
    autoRenew: true
})

await CertificateManager.renewLetsEncrypt({
    domains: ['example.com']
})

// ---------- 审计管理 ----------
await AuditManager.enable({
    rules: [
        {
            type: 'file',
            path: '/etc/passwd',
            permissions: ['read', 'write', 'execute', 'attribute']
        },
        {
            type: 'syscall',
            syscalls: ['open', 'unlink', 'rename']
        },
        {
            type: 'user',
            users: ['root'],
            actions: ['login', 'logout', 'failed-login']
        }
    ]
})

const logs = await AuditManager.queryLogs({
    filter: {
        type: 'file',
        user: 'root',
        action: 'write',
        since: '2024-01-01',
        until: '2024-12-31'
    },
    limit: 1000
})

const report = await AuditManager.generateReport({
    type: 'security',              // security, compliance, activity
    period: 'monthly',
    format: 'pdf', // pdf, html, json
    output: '/reports/audit-2024-01.pdf'
})

await AuditManager.exportLogs({
    destination: '/backup/audit-logs',
    format: 'json',
    compress: true
})

// ---------- 合规管理 ----------
const compliance = await ComplianceManager.check({
    standard: 'CIS',               // CIS, PCI-DSS, HIPAA, SOC2
    level: 1                       // 合规级别
})
// 返回: { passed, failed, warnings, score, details }

await ComplianceManager.remediate({
    standard: 'CIS',
    autoFix: true,                 // 自动修复
    rules: ['1.1.1', '1.1.2']      // 可选，指定规则
})

const baseline = await ComplianceManager.createBaseline({
    name: 'production-baseline',
    standard: 'CIS'
})

await ComplianceManager.compareBaseline({
    baseline: 'production-baseline',
    current: true
})


// ============================================
// 数据库管理
// ============================================

import {
    DatabaseManager,
    MySQLManager,
    PostgreSQLManager,
    MongoDBManager,
    RedisManager
} from 'raven/database'

// ---------- MySQL 管理 ----------
await MySQLManager.createDatabase({
    name: 'myapp',
    charset: 'utf8mb4',
    collation: 'utf8mb4_unicode_ci'
})

await MySQLManager.dropDatabase({
    name: 'myapp',
    force: false
})

await MySQLManager.createUser({
    username: 'appuser',
    password: 'secret',
    host: 'localhost'// 或 '%' 允许任何主机
})

await MySQLManager.grantPrivileges({
    user: 'appuser',
    host: 'localhost',
    database: 'myapp',
    privileges: ['SELECT', 'INSERT', 'UPDATE', 'DELETE'],
    tables: ['*']                  // 所有表
})

await MySQLManager.revokePrivileges({
    user: 'appuser',
    host: 'localhost',
    database: 'myapp',
    privileges: ['DELETE']
})

await MySQLManager.backup({
    databases: ['myapp'],
    output: '/backup/myapp.sql',
    compress: true,
    includeRoutines: true,         // 包含存储过程
    includeTriggers: true
})

await MySQLManager.restore({
    input: '/backup/myapp.sql',
    database: 'myapp'
})

const status = await MySQLManager.getStatus()
// 返回: { uptime, threads, queries, slowQueries, connections, ... }

const databases = await MySQLManager.listDatabases()

const users = await MySQLManager.listUsers()

await MySQLManager.optimize({
    database: 'myapp',
    tables: ['users', 'orders']    // 可选
})

await MySQLManager.repair({
    database: 'myapp',
    tables: ['users']
})

// ---------- PostgreSQL 管理 ----------
await PostgreSQLManager.createDatabase({
    name: 'myapp',
    owner: 'appuser',
    encoding: 'UTF8',
    locale: 'en_US.UTF-8'
})

await PostgreSQLManager.dropDatabase({
    name: 'myapp',
    force: false
})

await PostgreSQLManager.createUser({
    username: 'appuser',
    password: 'secret',
    superuser: false,
    createdb: false,
    createrole: false
})

await PostgreSQLManager.grantPrivileges({
    user: 'appuser',
    database: 'myapp',
    privileges: ['CONNECT', 'CREATE'],
    schema: 'public',
    tables: ['ALL']
})

await PostgreSQLManager.backup({
    database: 'myapp',
    output: '/backup/myapp.dump',
    format: 'custom',              // custom, plain, directory, tar
    compress: 9,                   // 0-9
    jobs: 4                        // 并行任务数
})

await PostgreSQLManager.restore({
    input: '/backup/myapp.dump',
    database: 'myapp',
    clean: true,                   // 恢复前清理
    jobs: 4
})

await PostgreSQLManager.vacuum({
    database: 'myapp',
    full: false,
    analyze: true
})

const activity = await PostgreSQLManager.getActivity()
// 返回: { connections, queries, locks, ... }

// ---------- MongoDB 管理 ----------
await MongoDBManager.createDatabase({
    name: 'myapp'
})

await MongoDBManager.dropDatabase({
    name: 'myapp'
})

await MongoDBManager.createUser({
    username: 'appuser',
    password: 'secret',
    database: 'myapp',
    roles: ['readWrite']           // read, readWrite, dbAdmin, userAdmin
})

await MongoDBManager.backup({
    database: 'myapp',
    output: '/backup/myapp',
    compress: true,
    oplog: true                    // 包含 oplog
})

await MongoDBManager.restore({
    input: '/backup/myapp',
    database: 'myapp',
    drop: true                     // 恢复前删除集合
})

const stats = await MongoDBManager.getStats({
    database: 'myapp'
})

await MongoDBManager.createIndex({
    database: 'myapp',
    collection: 'users',
    keys: {email: 1},
    unique: true
})

// ---------- Redis 管理 ----------
await RedisManager.backup({
    output: '/backup/dump.rdb'
})

await RedisManager.restore({
    input: '/backup/dump.rdb'
})

const info = await RedisManager.getInfo()

await RedisManager.flushDB({
    database: 0
})

await RedisManager.flushAll()

const keys = await RedisManager.keys({
    pattern: 'user:*'
})

await RedisManager.setConfig({
    key: 'maxmemory',
    value: '2gb'
})


// ============================================
// Web 服务器管理
// ============================================

import {
    NginxManager,
    ApacheManager,
    VirtualHostManager
} from 'raven/webserver'

// ---------- Nginx 管理 ----------
await NginxManager.createSite({
    name: 'example.com',
    config: {
        listen: [80, 443],
        serverName: ['example.com', 'www.example.com'],
        root: '/var/www/example.com',
        index: ['index.html', 'index.php'],
        ssl: {
            enabled: true,
            certificate: '/etc/ssl/certs/example.crt',
            certificateKey: '/etc/ssl/private/example.key',
            protocols: ['TLSv1.2', 'TLSv1.3']
        },
        locations: [
            {
                path: '/',
                tryFiles: ['$uri', '$uri/', '/index.php?$query_string']
            },
            {
                path: '~ \\.php$',
                fastcgiPass: 'unix:/var/run/php/php8.1-fpm.sock',
                fastcgiIndex: 'index.php',
                include: 'fastcgi_params'
            }
        ],
        accessLog: '/var/log/nginx/example.com-access.log',
        errorLog: '/var/log/nginx/example.com-error.log'
    }
})

await NginxManager.deleteSite({
    name: 'example.com',
    removeFiles: false// 是否删除网站文件
})

await NginxManager.enableSite({
    name: 'example.com'
})

await NginxManager.disableSite({
    name: 'example.com'
})

const sites = await NginxManager.listSites({
    enabled: true
})

await NginxManager.testConfig()    // 测试配置文件语法

await NginxManager.reload()        // 重载配置

await NginxManager.createUpstream({
    name: 'backend',
    servers: [
        {address: '127.0.0.1:3000', weight: 1},
        {address: '127.0.0.1:3001', weight: 1}
    ],
    method: 'least_conn'           // round_robin, least_conn, ip_hash
})

await NginxManager.createProxy({
    location: '/api',
    proxyPass: 'http://backend',
    headers: {
        'X-Real-IP': '$remote_addr',
        'X-Forwarded-For': '$proxy_add_x_forwarded_for'
    },
    timeout: 60
})

const logs = await NginxManager.analyzeLogs({
    file: '/var/log/nginx/access.log',
    metrics: ['requests', 'status', 'bandwidth', 'topIPs'],
    period: 'today'
})

// ---------- Apache 管理 ----------
await ApacheManager.createSite({
    name: 'example.com',
    config: {
        serverName: 'example.com',
        serverAlias: ['www.example.com'],
        documentRoot: '/var/www/example.com',
        port: 80,
        ssl: {
            enabled: true,
            certificate: '/etc/ssl/certs/example.crt',
            certificateKey: '/etc/ssl/private/example.key'
        },
        directory: {
            path: '/var/www/example.com',
            options: ['Indexes', 'FollowSymLinks'],
            allowOverride: 'All',
            require: 'all granted'
        },
        customLog: '/var/log/apache2/example.com-access.log combined',
        errorLog: '/var/log/apache2/example.com-error.log'
    }
})

await ApacheManager.enableModule({
    module: 'rewrite'// rewrite, ssl, headers, proxy
})

await ApacheManager.disableModule({
    module: 'status'
})

const modules = await ApacheManager.listModules({
    enabled: true
})

// ---------- 虚拟主机管理 ----------
await VirtualHostManager.create({
    type: 'nginx',                 // nginx, apache
    domain: 'example.com',
    root: '/var/www/example.com',
    php: '8.1',                    // PHP 版本
    ssl: true,
    letsencrypt: true,             // 自动申请 Let's Encrypt
    database: {
        type: 'mysql',
        name: 'example_db',
        user: 'example_user',
        password: 'auto'           // 自动生成密码
    }
})

await VirtualHostManager.delete({
    domain: 'example.com',
    removeFiles: true,
    removeDatabase: true
})

const vhosts = await VirtualHostManager.list()


// ============================================
// 邮件服务器管理
// ============================================

import {
    MailServerManager,
    PostfixManager,
    DovecotManager, MailAccountManager
} from 'raven/mail'

// ---------- Postfix 管理 ----------
await PostfixManager.configure({
    hostname: 'mail.example.com',
    domain: 'example.com',
    relayHost: 'smtp.gmail.com',   // 可选
    relayPort: 587,
    relayAuth: {
        username: 'user@gmail.com',
        password: 'secret'
    },
    tls: {
        enabled: true,
        certificate: '/etc/ssl/certs/mail.crt',
        key: '/etc/ssl/private/mail.key'
    },
    restrictions: {
        smtpd_recipient_restrictions: [
            'permit_mynetworks',
            'permit_sasl_authenticated',
            'reject_unauth_destination'
        ]
    }
})

await PostfixManager.reload()

const queue = await PostfixManager.getQueue()
// 返回: { active, deferred, hold, incoming }

await PostfixManager.flushQueue()

await PostfixManager.deleteFromQueue({
    messageId: 'ABC123'
})

const logs = await PostfixManager.analyzeLogs({
    period: 'today',
    metrics: ['sent', 'bounced', 'rejected', 'deferred']
})

// ---------- Dovecot 管理 ----------
await DovecotManager.configure({
    protocols: ['imap', 'pop3'],
    ssl: {
        enabled: true,
        certificate: '/etc/ssl/certs/mail.crt',
        key: '/etc/ssl/private/mail.key'
    },
    mailLocation: 'maildir:~/Maildir',
    auth: {
        mechanisms: ['plain', 'login'],
        userdb: 'passwd',
        passdb: 'pam'
    }
})

await DovecotManager.reload()

const connections = await DovecotManager.getConnections()

// ---------- 邮件账户管理 ----------
await MailAccountManager.createAccount({
    email: 'user@example.com',
    password: 'secret',
    quota: '1G',
    aliases: ['info@example.com']
})

await MailAccountManager.deleteAccount({
    email: 'user@example.com',
    removeMailbox: true
})

await MailAccountManager.setQuota({
    email: 'user@example.com',
    quota: '2G'
})

await MailAccountManager.addAlias({
    alias: 'support@example.com',
    destination: 'user@example.com'
})

const accounts = await MailAccountManager.listAccounts()

const usage = await MailAccountManager.getUsage({
    email: 'user@example.com'
})


// ============================================
// 集群管理
// ============================================

import {
    ClusterManager,
    KubernetesManager,
    LoadBalancerManager,
    HAManager
} from 'raven/cluster'

// ---------- Kubernetes 管理 ----------
await KubernetesManager.createDeployment({
    name: 'myapp',
    namespace: 'default',
    replicas: 3,
    image: 'myapp:1.0',
    ports: [{containerPort: 8080}],
    env: {
        ENV: 'production'
    },
    resources: {
        requests: {
            cpu: '100m',
            memory: '128Mi'
        },
        limits: {
            cpu: '500m',
            memory: '512Mi'
        }
    },
    livenessProbe: {
        httpGet: {
            path: '/health',
            port: 8080
        },
        initialDelaySeconds: 30,
        periodSeconds: 10
    }
})

await KubernetesManager.scaleDeployment({
    name: 'myapp',
    namespace: 'default',
    replicas: 5
})

await KubernetesManager.deleteDeployment({
    name: 'myapp',
    namespace: 'default'
})

await KubernetesManager.createService({
    name: 'myapp',
    namespace: 'default',
    type: 'LoadBalancer',          // ClusterIP, NodePort, LoadBalancer
    selector: {
        app: 'myapp'
    },
    ports: [
        {port: 80, targetPort: 8080}
    ]
})

await KubernetesManager.createIngress({
    name: 'myapp',
    namespace: 'default',
    rules: [
        {
            host: 'myapp.example.com',
            paths: [
                {
                    path: '/',
                    backend: {
                        serviceName: 'myapp',
                        servicePort: 80
                    }
                }
            ]
        }
    ],
    tls: [
        {
            hosts: ['myapp.example.com'],
            secretName: 'myapp-tls'
        }
    ]
})

const pods = await KubernetesManager.listPods({
    namespace: 'default',
    labelSelector: 'app=myapp'
})

const logs = await KubernetesManager.getPodLogs({
    pod: 'myapp-abc123',
    namespace: 'default',
    container: 'myapp',
    follow: false,
    tail: 100
})

await KubernetesManager.execInPod({
    pod: 'myapp-abc123',
    namespace: 'default',
    container: 'myapp',
    command: ['ls', '-la']
})

// ---------- 负载均衡管理 ----------
await LoadBalancerManager.create({
    name: 'web-lb',
    algorithm: 'round_robin',      // round_robin, least_conn, ip_hash
    backends: [
        {host: '192.168.1.10', port: 80, weight: 1},
        {host: '192.168.1.11', port: 80, weight: 1}
    ],
    healthCheck: {
        type: 'http',
        path: '/health',
        interval: 5,
        timeout: 3,
        unhealthyThreshold: 3
    },
    ssl: {
        enabled: true,
        certificate: '/etc/ssl/certs/lb.crt',
        key: '/etc/ssl/private/lb.key'
    }
})

await LoadBalancerManager.addBackend({
    name: 'web-lb',
    backend: {host: '192.168.1.12', port: 80, weight: 1}
})

await LoadBalancerManager.removeBackend({
    name: 'web-lb',
    backend: '192.168.1.12:80'
})

const stats = await LoadBalancerManager.getStats({
    name: 'web-lb'
})

// ---------- 高可用管理 ----------
await HAManager.createCluster({
    name: 'web-cluster',
    nodes: [
        {hostname: 'node1', ip: '192.168.1.10', priority: 100},
        {hostname: 'node2', ip: '192.168.1.11', priority: 90}
    ],
    virtualIP: '192.168.1.100',
    interface: 'eth0',
    authPassword: 'secret'
})

await HAManager.addNode({
    cluster: 'web-cluster',
    node: {hostname: 'node3', ip: '192.168.1.12', priority: 80}
})

await HAManager.removeNode({
    cluster: 'web-cluster',
    hostname: 'node3'
})

const status = await HAManager.getStatus({
    cluster: 'web-cluster'
})

await HAManager.failover({
    cluster: 'web-cluster',
    toNode: 'node2'
})


// ============================================
// 自动化与编排
// ============================================

import {
    AutomationManager,
    PlaybookManager,
    WorkflowManager,
    TemplateManager
} from 'raven/automation'

// ---------- Playbook 管理 ----------
await PlaybookManager.create({
    name: 'deploy-app',
    description: 'Deploy application to production',
    tasks: [
        {
            name: 'Update packages',
            module: 'PackageManager.update',
            params: {}
        },
        {
            name: 'Install dependencies',
            module: 'PackageManager.install',
            params: {
                packages: ['nginx', 'nodejs']
            }
        },
        {
            name: 'Deploy application',
            module: 'FileManager.copy',
            params: {
                source: '/tmp/app',
                destination: '/var/www/app'
            }
        },
        {
            name: 'Restart service',
            module: 'ServiceManager.restart',
            params: {
                service: 'nginx'
            }
        }
    ],
    variables: {
        app_version: '1.0.0',
        environment: 'production'
    },
    handlers: [
        {
            name: 'Rollback on failure',
            trigger: 'on_failure',
            tasks: [/* rollback tasks */]
        }
    ]
})

await PlaybookManager.run({
    name: 'deploy-app',
    variables: {
        app_version: '1.0.1'
    }, dryRun: false, // 试运行
    verbose: true
})

const runs = await PlaybookManager.listRuns({
    playbook: 'deploy-app',
    status: 'completed'
})

// ---------- 工作流管理 ----------
await WorkflowManager.create({
    name: 'ci-cd-pipeline',
    trigger: {
        type: 'webhook',           // webhook, schedule, manual
        url: '/webhook/deploy'
    },
    stages: [
        {
            name: 'Build',
            steps: [
                {action: 'git.clone', params: {repo: 'https://...'}},
                {action: 'docker.build', params: {tag: 'myapp:latest'}}
            ]
        },
        {
            name: 'Test',
            steps: [
                {action: 'docker.run', params: {image: 'myapp:latest', command: 'npm test'}}
            ]
        },
        {
            name: 'Deploy',
            steps: [
                {action: 'kubernetes.updateDeployment', params: {image: 'myapp:latest'}}
            ],
            approval: {
                required: true,
                approvers: ['admin@example.com']
            }
        }
    ],
    notifications: [
        {
            type: 'email',
            to: ['team@example.com'],
            on: ['success', 'failure']
        }
    ]
})

await WorkflowManager.trigger({
    name: 'ci-cd-pipeline',
    params: {
        branch: 'main',
        commit: 'abc123'
    }
})

const executions = await WorkflowManager.listExecutions({
    workflow: 'ci-cd-pipeline',
    status: 'running'
})

await WorkflowManager.approve({
    execution: 'exec-id',
    stage: 'Deploy',
    approver: 'admin@example.com'
})

// ---------- 模板管理 ----------
await TemplateManager.create({
    name: 'lamp-stack',
    description: 'LAMP stack template',
    parameters: [
        {name: 'domain', type: 'string', required: true},
        {name: 'phpVersion', type: 'string', default: '8.1'},
        {name: 'dbPassword', type: 'string', secret: true}
    ],
    resources: [
        {
            type: 'package',
            packages: ['apache2', 'mysql-server', 'php{{ phpVersion }}']
        },
        {
            type: 'virtualhost',
            domain: '{{ domain }}',
            root: '/var/www/{{ domain }}'
        },
        {
            type: 'database',
            name: '{{ domain | replace(".", "_") }}',
            user: '{{ domain | replace(".", "_") }}',
            password: '{{ dbPassword }}'
        }
    ]
})

await TemplateManager.deploy({
    template: 'lamp-stack',
    parameters: {
        domain: 'example.com',
        phpVersion: '8.2',
        dbPassword: 'secret123'
    }
})

const templates = await TemplateManager.list()


// ============================================
// 配置管理
// ============================================

import {
    ConfigManager,
    SecretManager,
    EnvironmentManager
} from 'raven/config'

// ---------- 配置管理 ----------
await ConfigManager.set({
    key: 'app.database.host',
    value: 'localhost',
    environment: 'production'      // 可选
})

const value = await ConfigManager.get({
    key: 'app.database.host',
    environment: 'production'
})

await ConfigManager.delete({
    key: 'app.database.host'
})

const configs = await ConfigManager.list({
    prefix: 'app.database',
    environment: 'production'
})

await ConfigManager.import({
    source: '/path/to/config.json',
    format: 'json',                // json, yaml, env
    environment: 'production',
    overwrite: false
})

await ConfigManager.export({
    destination: '/path/to/config.json',
    format: 'json',
    environment: 'production',
    keys: ['app.*']                // 可选，导出特定键
})

// ---------- 密钥管理 ----------
await SecretManager.create({
    name: 'db-password',
    value:
        'super-secret-password',
    description:
        'Database password',
    tags:
        ['database', 'production'],
    rotation:
        {
            enabled: true,
            interval:
                '90d'            // 90 天自动轮换
        }
})

const secret = await SecretManager.get({
    name: 'db-password',
    version: 'latest'              // 或指定版本号
})

await SecretManager.update({
    name: 'db-password',
    value: 'new-secret-password'
})

await SecretManager.delete({
    name: 'db-password',
    force: false
})

const secrets = await SecretManager.list({
    tags: ['production']
})

await SecretManager.rotate({
    name: 'db-password',
    newValue: 'rotated-password'
})

const history = await SecretManager.getHistory({
    name: 'db-password',
    limit: 10
})

await SecretManager.grantAccess({
    secret: 'db-password',
    principal: 'user:john',        // user:john, group:developers, service:myapp
    permissions: ['read']// read, write, delete
})

await SecretManager.revokeAccess({
    secret: 'db-password',
    principal: 'user:john'
})

// ---------- 环境管理 ----------
await EnvironmentManager.create({
    name: 'staging',
    description: 'Staging environment',
    variables: {
        NODE_ENV: 'staging',
        API_URL: 'https://api-staging.example.com',
        DB_HOST: 'db-staging.example.com'
    },
    secrets: {
        DB_PASSWORD: 'secret:db-password-staging',
        API_KEY: 'secret:api-key-staging'
    },
    inherits: 'production'         // 继承生产环境配置
})

await EnvironmentManager.delete({
    name: 'staging'
})

const env = await EnvironmentManager.get({
    name: 'staging'
})

await EnvironmentManager.setVariable({
    environment: 'staging',
    key: 'DEBUG',
    value: 'true'
})

const variables = await EnvironmentManager.listVariables({
    environment: 'staging'
})

await EnvironmentManager.clone({
    source: 'production',
    destination: 'staging-new',
    includeSecrets: false
})

await EnvironmentManager.compare({
    environment1: 'production',
    environment2: 'staging'
})
// 返回: { added, removed, modified }


// ============================================
// 性能优化
// ============================================

import {
    PerformanceManager,
    CacheManager,
    OptimizationManager,
    TuningManager
} from 'raven/performance'

// ---------- 性能分析 ----------
const profile = await PerformanceManager.profile({
    target: 'system',              // system, process, application
    duration: 60,                  // 秒
    metrics: ['cpu', 'memory', 'disk', 'network']
})

const bottlenecks = await PerformanceManager.findBottlenecks({
    threshold: {
        cpu: 80,                   // CPU 使用率 > 80%
        memory: 90,                // 内存使用率 > 90%
        diskIO: 100                // 磁盘 IO > 100 MB/s
    }
})

const recommendations = await PerformanceManager.getRecommendations({
    target: 'system'
})
// 返回: [{ type, severity, description, action }]

await PerformanceManager.benchmark({
    type: 'disk',                  // disk, cpu, memory, network
    duration: 60,
    output: '/tmp/benchmark-results.json'
})

// ---------- 缓存管理 ----------
await CacheManager.configure({
    type: 'redis',                 // redis, memcached, file
    host: 'localhost',
    port: 6379,
    ttl: 3600,                     // 默认 TTL（秒）
    maxSize: '1G'
})

await CacheManager.set({
    key: 'user:1000',
    value: {name: 'John', email: 'john@example.com'},
    ttl: 7200
})

const cached = await CacheManager.get({
    key: 'user:1000'
})

await CacheManager.delete({
    key: 'user:1000'
})

await CacheManager.flush({
    pattern: 'user:*'              // 删除匹配的键
})

const stats = await CacheManager.getStats()
// 返回: { hits, misses, hitRate, size, keys }

await CacheManager.warm({
    keys: ['user:1000', 'user:1001'],
    source: 'database'
})

// ---------- 优化管理 ----------
await OptimizationManager.optimizeDatabase({
    type: 'mysql',
    database: 'myapp',
    actions: ['analyze', 'optimize', 'repair']
})

await OptimizationManager.optimizeImages({
    path: '/var/www/images',
    quality: 85,
    format: 'webp',                // 转换格式
    recursive: true,
    backup: true
})

await OptimizationManager.compressAssets({
    path: '/var/www/assets',
    types: ['js', 'css', 'html'],
    minify: true,
    gzip: true,
    brotli: true
})

await OptimizationManager.cleanupLogs({
    path: '/var/log',
    olderThan: '30d',
    compress: true,
    delete: false
})

await OptimizationManager.cleanupPackages({
    removeUnused: true,
    removeOldKernels: true,
    cleanCache: true
})

// ---------- 系统调优 ----------
await TuningManager.tuneKernel({
    parameters: {
        'vm.swappiness': 10,
        'net.core.somaxconn': 65535,
        'net.ipv4.tcp_max_syn_backlog': 8192,
        'fs.file-max': 2097152
    },
    persistent: true               // 写入 /etc/sysctl.conf
})

await TuningManager.tuneMySQL({
    parameters: {
        'innodb_buffer_pool_size': '4G',
        'max_connections': 500,
        'query_cache_size': '256M'
    }
})

await TuningManager.tuneNginx({
    parameters: {
        'worker_processes': 'auto',
        'worker_connections': 4096,
        'keepalive_timeout': 65
    }
})

const suggestions = await TuningManager.analyze({
    target: 'system',
    autoApply: false
})


// ============================================
// 灾难恢复
// ============================================

import {
    DisasterRecoveryManager,
    ReplicationManager,
    FailoverManager,
    DRTestManager
} from 'raven/disaster-recovery'

// ---------- 灾难恢复计划 ----------
await DisasterRecoveryManager.createPlan({
    name: 'production-dr',
    rpo: 3600,                     // Recovery Point Objective (秒)
    rto: 7200,                     // Recovery Time Objective (秒)
    primarySite: {
        location: 'datacenter-1',
        servers: ['web-1', 'web-2', 'db-1']
    },
    drSite: {
        location: 'datacenter-2',
        servers: ['web-dr-1', 'web-dr-2', 'db-dr-1']
    },
    replication: {
        type: 'async',             // sync, async
        interval: 300              // 秒
    },
    procedures: [
        {
            step: 1,
            action: 'Stop primary services',
            commands: ['systemctl stop nginx', 'systemctl stop mysql']
        },
        {
            step: 2,
            action: 'Activate DR site',
            commands: ['systemctl start nginx', 'systemctl start mysql']
        },
        {
            step: 3,
            action: 'Update DNS',
            commands: ['update-dns.sh']
        }
    ]
})

await DisasterRecoveryManager.activate({
    plan: 'production-dr',
    reason: 'Primary site failure',
    notify: ['admin@example.com']
})

await DisasterRecoveryManager.deactivate({
    plan: 'production-dr',
    switchback: true               // 切回主站点
})

const status = await DisasterRecoveryManager.getStatus({
    plan: 'production-dr'
})

// ---------- 数据复制管理 ----------
await ReplicationManager.setup({
    type: 'mysql',
    master: {
        host: '192.168.1.10',
        port: 3306,
        user: 'repl',
        password: 'secret'
    },
    slave: {
        host: '192.168.2.10',
        port: 3306
    },
    databases: ['myapp'],
    mode: 'async'                  // sync, async, semi-sync
})

await ReplicationManager.start({
    type: 'mysql',
    slave: '192.168.2.10'
})

await ReplicationManager.stop({
    type: 'mysql',
    slave: '192.168.2.10'
})

const replStatus = await ReplicationManager.getStatus({
    type: 'mysql',
    slave: '192.168.2.10'
})
// 返回: { running, lag, position, errors }

await ReplicationManager.resync({
    type: 'mysql',
    slave: '192.168.2.10',
    full: false                    // 完全重新同步
})

// ---------- 故障转移管理 ----------
await FailoverManager.configure({
    service: 'database',
    primary: '192.168.1.10',
    standby: ['192.168.2.10', '192.168.3.10'],
    healthCheck: {
        interval: 5,
        timeout: 3,
        retries: 3
    },
    automatic: true,               // 自动故障转移
    vip: '192.168.1.100'          // 虚拟 IP
})

await FailoverManager.trigger({
    service: 'database',
    from: '192.168.1.10',
    to: '192.168.2.10',
    reason: 'Manual failover for maintenance'
})

await FailoverManager.switchback({
    service: 'database',
    to: '192.168.1.10'
})

const failoverHistory = await FailoverManager.getHistory({
    service: 'database',
    limit: 50
})

// ---------- 灾难恢复测试 ----------
await DRTestManager.schedule({
    plan: 'production-dr',
    schedule: '0 2 * * 0',         // 每周日凌晨 2 点
    scope: 'partial',              // full, partial
    notify: ['team@example.com']
})

await DRTestManager.run({
    plan: 'production-dr',
    scope: 'full',
    dryRun: false
})

const testResults = await DRTestManager.getResults({
    plan: 'production-dr',
    limit: 10
})

const report = await DRTestManager.generateReport({
    plan: 'production-dr',
    testId: 'test-123',
    format: 'pdf'
})


// ============================================
// 合规与审计
// ============================================

import {
    ComplianceManager,
    AuditLogManager,
    PolicyManager,
    ReportManager
} from 'raven/compliance'

// ---------- 合规检查 ----------
await ComplianceManager.runCheck({
    framework: 'PCI-DSS',          // PCI-DSS, HIPAA, SOC2, GDPR, ISO27001
    scope: 'all',                  // all, network, system, application
    autoRemediate: false
})

const results = await ComplianceManager.getResults({
    framework: 'PCI-DSS',
    status: 'failed'               // passed, failed, warning
})

await ComplianceManager.remediate({
    framework: 'PCI-DSS',
    controls: ['1.1.1', '1.1.2'],
    approve: true
})

await ComplianceManager.createException({
    framework: 'PCI-DSS',
    control: '1.1.1',
    reason: 'Legacy system, scheduled for replacement',
    expiresAt: '2025-12-31',
    approver: 'ciso@example.com'
})

// ---------- 审计日志管理 ----------
await AuditLogManager.log({
    event: 'user.login',
    actor: 'john@example.com',
    resource: 'system',
    action: 'login',
    result: 'success',
    metadata: {
        ip: '192.168.1.100',
        userAgent: 'Mozilla/5.0...'
    }
})

const logs = await AuditLogManager.query({
    filter: {
        event: 'user.*',
        actor: 'john@example.com',
        result: 'failure',
        since: '2024-01-01',
        until: '2024-12-31'
    },
    limit: 1000,
    sort: 'desc'
})

await AuditLogManager.export({
    filter: {
        since: '2024-01-01',
        until: '2024-12-31'
    },
    format: 'json',
    destination: '/backup/audit-logs-2024.json',
    encrypt: true
})

const integrity = await AuditLogManager.verifyIntegrity({
    since: '2024-01-01',
    until: '2024-12-31'
})

// ---------- 策略管理 ----------
await PolicyManager.create({
    name: 'password-policy',
    type: 'security',
    rules: [
        {
            name: 'minimum-length',
            value: 12
        },
        {
            name: 'require-uppercase',
            value: true
        },
        {
            name: 'require-numbers',
            value: true
        },
        {
            name: 'require-special-chars',
            value: true
        },
        {
            name: 'max-age-days',
            value: 90
        }
    ],
    enforcement: 'strict',         // strict, advisory
    scope: 'all-users'
})

await PolicyManager.apply({
    policy: 'password-policy',
    targets: ['all-users']
})

await PolicyManager.validate({
    policy: 'password-policy',
    target: 'user:john'
})

const violations = await PolicyManager.findViolations({
    policy: 'password-policy'
})

// ---------- 报告管理 ----------
await ReportManager.generate({
    type: 'compliance',            // compliance, security, audit, performance
    framework: 'PCI-DSS',
    period: {
        start: '2024-01-01',
        end: '2024-12-31'
    },
    format: 'pdf',
    sections: [
        'executive-summary',
        'findings',
        'recommendations',
        'evidence'
    ],
    output: '/reports/pci-dss-2024.pdf'
})

await ReportManager.schedule({
    type: 'security',
    frequency: 'monthly',
    recipients: ['security@example.com'],
    format: 'pdf'
})

const reports = await ReportManager.list({
    type: 'compliance',
    since: '2024-01-01'
})


// ============================================
// 成本管理
// ============================================

import {
    CostManager,
    ResourceUsageManager,
    BudgetManager,
    OptimizationRecommendations
} from 'raven/cost'

// ---------- 成本跟踪 ----------
const costs = await CostManager.getCosts({
    period: 'monthly',
    groupBy: 'service',            // service, resource, tag
    since: '2024-01-01',
    until: '2024-12-31'
})

const forecast = await CostManager.forecast({
    period: 'next-month',
    basedOn: 'last-3-months'
})

await CostManager.setTags({
    resource: 'server-1',
    tags: {
        environment: 'production',
        project: 'web-app',
        costCenter: 'engineering'
    }
})

const breakdown = await CostManager.getBreakdown({
    period: 'monthly',
    groupBy: ['service', 'environment']
})

// ---------- 资源使用管理 ----------
const usage = await ResourceUsageManager.getUsage({
    resource: 'compute',           // compute, storage, network, database
    period: 'daily',
    since: '2024-01-01'
})

const idle = await ResourceUsageManager.findIdle({
    threshold: {
        cpu: 5,                    // CPU < 5%
        duration: '7d'             // 持续 7 天
    }
})

const underutilized = await ResourceUsageManager.findUnderutilized({
    threshold: {
        cpu: 20,
        memory: 30,
        duration: '30d'
    }
})

// ---------- 预算管理 ----------
await BudgetManager.create({
    name: 'engineering-2024',
    amount: 100000,
    period: 'yearly',
    alerts: [
        {threshold: 50, notify: ['manager@example.com']},
        {threshold: 80, notify: ['cfo@example.com']},
        {threshold: 100, notify: ['ceo@example.com']}
    ],
    filters: {
        tags: {
            costCenter: 'engineering'
        }
    }
})

const budgetStatus = await BudgetManager.getStatus({
    name: 'engineering-2024'
})
// 返回: { spent, remaining, percentage, forecast }

const alerts = await BudgetManager.getAlerts({
    status: 'triggered'
})

// ---------- 优化建议 ----------
const recommendations = await OptimizationRecommendations.get({
    category: 'all',               // compute, storage, network, database
    minSavings: 100// 最小节省金额
})
// 返回: [{ type, resource, currentCost, potentialSavings, action }]

await OptimizationRecommendations.apply({
    recommendationId: 'rec-123',
    autoApply: false
})


// ============================================
// 通知与告警
// ============================================

import {
    NotificationManager,
    AlertChannelManager,
    EscalationManager
} from 'raven/notification'

// ---------- 通知管理 ----------
await NotificationManager.send({
    channel: 'email',              // email, slack, webhook, sms
    to: ['admin@example.com'],
    subject: 'System Alert',
    message: 'High CPU usage detected',
    priority: 'high',              // low, normal, high, critical
    metadata: {
        server: 'web-1',
        cpu: 95
    }
})

await NotificationManager.sendBatch({
    notifications: [
        {channel: 'email', to: ['admin@example.com'], message: '...'},
        {channel: 'slack', to: ['#alerts'], message: '...'}
    ]
})

const history = await NotificationManager.getHistory({
    channel: 'email',
    since: '2024-01-01',
    status: 'delivered'            // pending, delivered, failed
})

// ---------- 告警通道管理 ----------
await AlertChannelManager.create({
    name: 'ops-email',
    type: 'email',
    config: {
        smtp: {
            host: 'smtp.gmail.com',
            port: 587,
            secure: true,
            auth: {
                user: 'alerts@example.com',
                password: 'secret'
            }
        },
        from: 'alerts@example.com',
        to: ['ops@example.com']
    },
    enabled: true
})

await AlertChannelManager.create({
    name: 'ops-slack',
    type: 'slack',
    config: {
        webhookUrl: 'https://hooks.slack.com/services/...',
        channel: '#ops-alerts',
        username: 'AlertBot'
    },
    enabled: true
})

await AlertChannelManager.create({
    name: 'ops-webhook',
    type: 'webhook',
    config: {
        url: 'https://api.example.com/alerts',
        method: 'POST',
        headers: {
            'Authorization': 'Bearer token',
            'Content-Type': 'application/json'
        }
    },
    enabled: true
})

await AlertChannelManager.test({
    name: 'ops-email'
})

// ---------- 升级管理 ----------
await EscalationManager.createPolicy({
    name: 'critical-alerts',
    levels: [
        {
            level: 1,
            delay: 0,                  // 立即通知
            channels: ['ops-slack'],
            recipients: ['oncall@example.com']
        },
        {
            level: 2,
            delay: 300,                // 5 分钟后升级
            channels: ['ops-email', 'sms'],
            recipients: ['manager@example.com']
        },
        {
            level: 3,
            delay: 900,                // 15 分钟后再次升级
            channels: ['phone'],
            recipients: ['director@example.com']
        }
    ],
    conditions: {
        severity: 'critical',
        unacknowledged: true
    }
})

await EscalationManager.trigger({
    policy: 'critical-alerts',
    alert: {
        id: 'alert-123',
        message: 'Database server down',
        severity: 'critical'
    }
})


// ============================================
// API 管理与网关
// ============================================

import {
    APIGatewayManager,
    RateLimitManager,
    APIKeyManager
} from 'raven/api'

// ---------- API 网关管理 ----------
await APIGatewayManager.createRoute({
    path: '/api/v1/users',
    methods: ['GET', 'POST'],
    upstream: 'http://backend:3000/users',
    middleware: [
        'auth',
        'rate-limit',
        'logging'
    ],
    timeout: 30,
    retries: 3
})

await APIGatewayManager.createUpstream({
    name: 'backend',
    servers: [
        {url: 'http://192.168.1.10:3000', weight: 1},
        {url: 'http://192.168.1.11:3000', weight: 1}
    ],
    healthCheck: {
        path: '/health',
        interval: 10,
        timeout: 5
    },
    loadBalancing: 'round-robin'
})

await APIGatewayManager.addMiddleware({
    name: 'auth',
    type: 'jwt',
    config: {
        secret: 'your-secret-key',
        algorithms: ['HS256']
    }
})

// ---------- 限流管理 ----------
await RateLimitManager.create({
    name: 'api-limit',
    limit: 1000,                   // 请求数
    window: 3600,                  // 时间窗口（秒）
    scope: 'ip',                   // ip, user, api-key
    action: 'reject',              // reject, throttle
    routes: ['/api/*']
})

await RateLimitManager.setQuota({
    apiKey: 'key-123',
    limit: 10000,
    window: 86400                  // 每天 10000 次
})

const usage = await RateLimitManager.getUsage({
    apiKey: 'key-123',
    period: 'today'
})

// ---------- API 密钥管理 ----------
await APIKeyManager.create({
    name: 'mobile-app',
    scopes: ['read:users', 'write:users'],
    rateLimit: {
        limit: 5000,
        window: 3600
    },
    expiresAt: '2025-12-31',
    metadata: {
        app: 'mobile',
        version: '1.0'
    }
})

await APIKeyManager.revoke({
    key: 'key-123',
    reason: 'Security breach'
})

await APIKeyManager.rotate({
    key: 'key-123',
    gracePeriod: 86400             // 旧密钥保留 24 小时
})

const keys = await APIKeyManager.list({
    status: 'active'
})


// ============================================
// 日志聚合与分析
// ============================================

import {
    LogAggregator,
    LogParser,
    LogAnalyzer
} from 'raven/logging'

// ---------- 日志聚合 ----------
await LogAggregator.addSource({
    name: 'nginx-access',
    type: 'file',
    path: '/var/log/nginx/access.log',
    format: 'combined',
    tags: ['nginx', 'web', 'access']
})

await LogAggregator.addSource({
    name: 'app-logs',
    type: 'syslog',
    host: '0.0.0.0',
    port: 514,
    protocol: 'udp',
    tags: ['application']
})

await LogAggregator.addSource({
    name: 'docker-logs',
    type: 'docker',
    containers: ['myapp', 'nginx'],
    tags: ['docker']
})

const logs = await LogAggregator.search({
    query: 'error OR exception',
    filters: {
        tags: ['application'],
        level: ['error', 'critical'],
        since: '2024-01-01',
        until: '2024-12-31'
    },
    limit: 1000,
    sort: 'desc'
})

// ---------- 日志解析 ----------
await LogParser.addPattern({
    name: 'custom-app-log',
    pattern: '%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}',
    fields: {
        timestamp: 'date',
        level: 'string',
        message: 'text'
    }
})

const parsed = await LogParser.parse({
    source: 'app-logs',
    pattern: 'custom-app-log'
})

// ---------- 日志分析 ----------
const analysis = await LogAnalyzer.analyze({
    source: 'nginx-access',
    metrics: [
        'requests-per-minute',
        'status-codes',
        'top-ips',
        'top-urls',
        'response-times'
    ],
    period: 'last-24-hours'
})

const anomalies = await LogAnalyzer.detectAnomalies({
    source: 'app-logs',
    baseline: 'last-7-days',
    sensitivity: 'medium'          // low, medium, high
})

await LogAnalyzer.createDashboard({
    name: 'Web Traffic',
    sources: ['nginx-access'],
    widgets: [
        {type: 'timeseries', metric: 'requests-per-minute'},
        {type: 'pie', metric: 'status-codes'},
        {type: 'table', metric: 'top-urls'}
    ]
})