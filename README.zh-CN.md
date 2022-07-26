Language : [🇺🇸 English](./README.md) | 🇨🇳 简体中文

<h1 align="center">ostat</h1>

## 简介
`ostat` 是用于统计系统数据（如cpu、memory、网络、磁盘）的 NodeJS 库

## 特点
- CPU 负载
- 系统平均负荷
- 内存使用
- 正常运行时间 / 启动时间
- 电池寿命
- 文件系统装载 (和磁盘使用率)
- 磁盘输入 / 输出统计信息
- 网络接口
- 网络流量统计
- CPU温度

## 目录
- [简介](#简介)
- [特点](#特点)
- [目录](#目录)
- [开始使用](#开始使用)
  - [先决条件](#先决条件)
  - [支持的系统](#支持的系统)
  - [安装](#安装)
- [文档](#文档)
  - [***`mounts`*** (Stat 类实例方法)](#mounts-stat-类实例方法)
  - [***`mountAt`*** (Stat 类实例方法)](#mountat-stat-类实例方法)
  - [***`blockDeviceStatistics`*** (Stat 类实例方法)](#blockdevicestatistics-stat-类实例方法)
  - [***`networks`*** (Stat 类实例方法)](#networks-stat-类实例方法)
  - [***`networkStats`*** (Stat 类实例方法)](#networkstats-stat-类实例方法)
  - [***`batteryLife`*** (Stat 类实例方法)](#batterylife-stat-类实例方法)
  - [***`isAcPower`*** (Stat 类实例方法)](#isacpower-stat-类实例方法)
  - [***`memory`*** (Stat 类实例方法)](#memory-stat-类实例方法)
  - [***`swap`*** (Stat 类实例方法)](#swap-stat-类实例方法)
  - [***`loadAverage`*** (Stat 类实例方法)](#loadaverage-stat-类实例方法)
  - [***`uptime`*** (Stat 类实例方法)](#uptime-stat-类实例方法)
  - [***`bootTime`*** (Stat 类实例方法)](#boottime-stat-类实例方法)
  - [***`cpuLoadAggregate`*** (Stat 类实例方法)](#cpuloadaggregate-stat-类实例方法)
  - [***`cpuTemp`*** (Stat 类实例方法)](#cputemp-stat-类实例方法)
  - [***`socketStats`*** (Stat 类实例方法)](#socketstats-stat-类实例方法)
  - [***`format`*** (方法)](#format-方法)
  - [***`FileSystem`*** (类型)](#filesystem-类型)
  - [***`BlockDeviceStats`*** (类型)](#blockdevicestats-类型)
  - [***`Network`*** (类型)](#network-类型)
  - [***`NetworkStats`*** (类型)](#networkstats-类型)
  - [***`BatteryLife`*** (类型)](#batterylife-类型)
  - [***`Memory`*** (类型)](#memory-类型)
  - [***`LoadAverage`*** (类型)](#loadaverage-类型)
  - [***`CPULoad`*** (类型)](#cpuload-类型)
  - [***`ConvertResult`*** (类型)](#convertresult-类型)
  - [***`ConvertOption`*** (类型)](#convertoption-类型)
- [如何开发](#如何开发)
  - [开发环境要求](#开发环境要求)
  - [本地测试](#本地测试)
- [发行说明](#发行说明)
- [感谢](#感谢)


## 开始使用
### 先决条件
- [Node.js](https://nodejs.org) (>= 10.4.0 required, LTS preferred)

### 支持的系统
|                       | node12 | node14 | node16 | node18 |
| --------------------- | ------ | ------ | ------ | ------ |
| Windows x64           | ✓      | ✓      | ✓      | ✓      |
| Windows x86           | ✓      | ✓      | ✓      | ✓      |
| Windows arm64         | ✓      | ✓      | ✓      | ✓      |
| macOS x64             | ✓      | ✓      | ✓      | ✓      |
| macOS aarch64         | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu         | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl        | ✓      | ✓      | ✓      | ✓      |
| Linux aarch64 gnu     | ✓      | ✓      | ✓      | ✓      |
| Linux aarch64 musl    | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64           | ✓      | ✓      | ✓      | ✓      |

### 安装
```bash
npm install ostat
# or
yarn add ostat
# or
pnpm i ostat
```

## 文档
首先需要创建 `Stat` 类实例
```typescript
const stat = new Stat()
```
### ***`mounts`*** (Stat 类实例方法)
获取文件系统装载信息
```typescript
mounts(): Array<FileSystem>
```

### ***`mountAt`*** (Stat 类实例方法)
获取给定路径下文件系统的文件系统装载信息
```typescript
mountAt(at: string): FileSystem
```

### ***`blockDeviceStatistics`*** (Stat 类实例方法)
获取块设备统计信息
```typescript
blockDeviceStatistics(): Array<BlockDeviceStats>
```

### ***`networks`*** (Stat 类实例方法)
获取网络信息
```typescript
networks(): Array<Network>
```

### ***`networkStats`*** (Stat 类实例方法)
获取给定接口名称的统计信息 (发送/接收的字节/数据包)
```typescript
networkStats(interface: string): NetworkStats
```

### ***`batteryLife`*** (Stat 类实例方法)
获取电池寿命信息
```typescript
batteryLife(): BatteryLife
```

### ***`isAcPower`*** (Stat 类实例方法)
获取交流电源是否已插入
```typescript
isAcPower(): boolean
```

### ***`memory`*** (Stat 类实例方法)
获取内存使用信息
```typescript
memory(): Memory
```

### ***`swap`*** (Stat 类实例方法)
获取交换内存的信息
```typescript
swap(): Memory
```

### ***`loadAverage`*** (Stat 类实例方法)
获取系统平均负载
```typescript
loadAverage(): LoadAverage
```

### ***`uptime`*** (Stat 类实例方法)
获取系统正常运行时间
```typescript
uptime(): bigint
```

### ***`bootTime`*** (Stat 类实例方法)
获取系统启动时间
```typescript
bootTime(): Date
```

### ***`cpuLoadAggregate`*** (Stat 类实例方法)
获取 CPU 负载统计信息, 平均所有 CPU (核心)
```typescript
cpuLoadAggregate(): CPULoad
```

### ***`cpuTemp`*** (Stat 类实例方法)
获取当前CPU温度 (摄氏度)

根据平台的不同，这可能是核心0、包等
```typescript
cpuTemp(): number
```

### ***`socketStats`*** (Stat 类实例方法)
获取正在使用的套接字数量的信息
```typescript
socketStats(): SocketStats
```

### ***`format`*** (方法)
根据 ConvertOption 格式化 source

默认使用 1024 进制, 保留一位小数
```typescript
function format(source: bigint, option?: ConvertOption | undefined | null): ConvertResult
```

### ***`FileSystem`*** (类型)
```typescript
interface FileSystem {
  /** 文件系统中使用的文件节点 */
  files: bigint
  /** 文件系统中的文件节点总数 */
  filesTotal: bigint
  /** 非超级用户可用的空闲节点 */
  filesAvail: bigint
  /** 文件系统中的可用字节 */
  free: bigint
  /** 非超级用户可用的可用字节 */
  avail: bigint
  /** 文件系统中的总字节数 */
  total: bigint
  /** 最大文件名长度 */
  nameMax: bigint
  fsType: string
  fsMountedFrom: string
  fsMountedOn: string
}
```

### ***`BlockDeviceStats`*** (类型)
```typescript
interface BlockDeviceStats {
  name: string
  readIos: bigint
  readMerges: bigint
  readSectors: bigint
  readTicks: bigint
  writeIos: bigint
  writeMerges: bigint
  writeSectors: bigint
  writeTicks: bigint
  inFlight: bigint
  ioTicks: bigint
  timeInQueue: bigint
}
```

### ***`Network`*** (类型)
```typescript
interface Network {
  name: string
  addrs: Array<NetworkAddrs>
}
```

```typescript
interface NetworkAddrs {
  addr: string
  netmask: string
  addrType: AddrType
}
```

```typescript
const enum AddrType {
  Ipv4 = 0,
  IPv6 = 1,
}
```

### ***`NetworkStats`*** (类型)
```typescript
interface NetworkStats {
  rxBytes: bigint
  txBytes: bigint
  rxPackets: bigint
  txPackets: bigint
  rxErrors: bigint
  txErrors: bigint
}
```

### ***`BatteryLife`*** (类型)
```typescript
interface BatteryLife {
  remainingCapacity: number
  remainingTime: bigint
}
```

### ***`Memory`*** (类型)
```typescript
interface Memory {
  free: bigint
  total: bigint
  used: bigint
}
```

### ***`LoadAverage`*** (类型)
```typescript
interface LoadAverage {
  one: number
  five: number
  fifteen: number
}
```

### ***`CPULoad`*** (类型)
```typescript
interface CPULoad {
  user: number
  nice: number
  system: number
  interrupt: number
  idle: number
}
```

### ***`ConvertResult`*** (类型)
```typescript
interface ConvertResult {
  value: string
  unit: string
}
```

### ***`ConvertOption`*** (类型)
```typescript
interface ConvertOption {
  advance: Advance
  /** 小数点位数 */
  precision: number
}
```

```typescript
/**
 * unit enum for stat option
 * B -> KB, 1000 bytes
 * B -> KIB, 1024 bytes
 */
export const enum Advance {
  /** 1000 */
  Kilobase = 1000,
  /** 1024 */
  Binary = 1024,
}
```


## 如何开发

### 开发环境要求

- Install the latest `Rust`
- Install `Node.js@10+` which fully supported `Node-API`
- Install `yarn@1.x`

### 本地测试

- yarn
- yarn build
- yarn test

## 发行说明
SEE [CHANGELOG](./CHANGELOG.md)

## 感谢
[systemstat](https://github.com/unrelentingtech/systemstat)

[napi-rs](https://github.com/napi-rs/napi-rs)
