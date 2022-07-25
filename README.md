Language : 🇺🇸 English | [🇨🇳 简体中文](./README.zh-CN.md)

<h1 align="center">ostat</h1>

## Overview
`ostat` is a NodeJS lib for statistical system data such as cpu, mem, network, disk.

## Features
- CPU load
- load average
- memory usage
- uptime / boot time
- battery life
- filesystem mounts (and disk usage)
- disk I/O statistics
- network interfaces
- network traffic statistics
- CPU temperature

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Table of Contents](#table-of-contents)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Support Operating Systems](#support-operating-systems)
  - [Installation](#installation)
- [Documentation](#documentation)
  - [***`mounts`*** (Stat class instance method)](#mounts-stat-class-instance-method)
  - [***`mountAt`*** (Stat class instance method)](#mountat-stat-class-instance-method)
  - [***`blockDeviceStatistics`*** (Stat class instance method)](#blockdevicestatistics-stat-class-instance-method)
  - [***`networks`*** (Stat class instance method)](#networks-stat-class-instance-method)
  - [***`networkStats`*** (Stat class instance method)](#networkstats-stat-class-instance-method)
  - [***`batteryLife`*** (Stat class instance method)](#batterylife-stat-class-instance-method)
  - [***`isAcPower`*** (Stat class instance method)](#isacpower-stat-class-instance-method)
  - [***`memory`*** (Stat class instance method)](#memory-stat-class-instance-method)
  - [***`swap`*** (Stat class instance method)](#swap-stat-class-instance-method)
  - [***`loadAverage`*** (Stat class instance method)](#loadaverage-stat-class-instance-method)
  - [***`uptime`*** (Stat class instance method)](#uptime-stat-class-instance-method)
  - [***`bootTime`*** (Stat class instance method)](#boottime-stat-class-instance-method)
  - [***`cpuLoadAggregate`*** (Stat class instance method)](#cpuloadaggregate-stat-class-instance-method)
  - [***`cpuTemp`*** (Stat class instance method)](#cputemp-stat-class-instance-method)
  - [***`socketStats`*** (Stat class instance method)](#socketstats-stat-class-instance-method)
  - [***`format`*** (Function)](#format-function)
  - [***`FileSystem`*** (Type)](#filesystem-type)
  - [***`BlockDeviceStats`*** (Type)](#blockdevicestats-type)
  - [***`Network`*** (Type)](#network-type)
  - [***`NetworkStats`*** (Type)](#networkstats-type)
  - [***`BatteryLife`*** (Type)](#batterylife-type)
  - [***`Memory`*** (Type)](#memory-type)
  - [***`LoadAverage`*** (Type)](#loadaverage-type)
  - [***`CPULoad`*** (Type)](#cpuload-type)
  - [***`ConvertResult`*** (Type)](#convertresult-type)
  - [***`ConvertOption`*** (Type)](#convertoption-type)
- [How to Develop](#how-to-develop)
  - [Development requirements](#development-requirements)
  - [Test in local](#test-in-local)
- [Release Notes](#release-notes)
- [Thanks](#thanks)


## Getting Started
### Prerequisites
- [Node.js](https://nodejs.org) (>= 10.4.0 required, LTS preferred)

### Support Operating Systems
|                  | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

### Installation
```bash
npm install ostat
# or
yarn add ostat
# or
pnpm i ostat
```

## Documentation
Create `Stat` class instance in first
```typescript
const stat = new Stat()
```
### ***`mounts`*** (Stat class instance method)
Get filesystem mount information.
```typescript
mounts(): Array<FileSystem>
```

### ***`mountAt`*** (Stat class instance method)
Get a filesystem mount information for the filesystem at a given path.
```typescript
mountAt(at: string): FileSystem
```

### ***`blockDeviceStatistics`*** (Stat class instance method)
Get block device statistics objects.
```typescript
blockDeviceStatistics(): Array<BlockDeviceStats>
```

### ***`networks`*** (Stat class instance method)
Get network intefrace information.
```typescript
networks(): Array<Network>
```

### ***`networkStats`*** (Stat class instance method)
Get statistics for a given interface name (bytes/packets sent/received).
```typescript
networkStats(interface: string): NetworkStats
```

### ***`batteryLife`*** (Stat class instance method)
Get a battery life information object.
```typescript
batteryLife(): BatteryLife
```

### ***`isAcPower`*** (Stat class instance method)
Get whether AC power is plugged in.
```typescript
isAcPower(): boolean
```

### ***`memory`*** (Stat class instance method)
Get memory information.
```typescript
memory(): Memory
```

### ***`swap`*** (Stat class instance method)
Get swap memory information.
```typescript
swap(): Memory
```

### ***`loadAverage`*** (Stat class instance method)
Get load average.
```typescript
loadAverage(): LoadAverage
```

### ***`uptime`*** (Stat class instance method)
Get the system uptime.
```typescript
uptime(): bigint
```

### ***`bootTime`*** (Stat class instance method)
Get the system boot time.
```typescript
bootTime(): Date
```

### ***`cpuLoadAggregate`*** (Stat class instance method)
Get CPU load statistics, average over all CPUs (cores).
```typescript
cpuLoadAggregate(): CPULoad
```

### ***`cpuTemp`*** (Stat class instance method)
Get the current CPU temperature in degrees Celsius.
Depending on the platform, this might be core 0, package, etc.
```typescript
cpuTemp(): number
```

### ***`socketStats`*** (Stat class instance method)
Get information about the number of sockets in use
```typescript
socketStats(): SocketStats
```

### ***`format`*** (Function)
Format the source by ConvertOption

The default of ConvertOption is taking ***1024*** as the advancement, retain ***one decimal*** place
```typescript
function format(source: bigint, option?: ConvertOption | undefined | null): ConvertResult
```

### ***`FileSystem`*** (Type)
```typescript
interface FileSystem {
  /** Used file nodes in filesystem */
  files: bigint
  /** Total file nodes in filesystem */
  filesTotal: bigint
  /** Free nodes available to non-superuser */
  filesAvail: bigint
  /** Free bytes in filesystem */
  free: bigint
  /** Free bytes available to non-superuser */
  avail: bigint
  /** Total bytes in filesystem */
  total: bigint
  /** Maximum filename length */
  nameMax: bigint
  fsType: string
  fsMountedFrom: string
  fsMountedOn: string
}
```

### ***`BlockDeviceStats`*** (Type)
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

### ***`Network`*** (Type)
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

### ***`NetworkStats`*** (Type)
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

### ***`BatteryLife`*** (Type)
```typescript
interface BatteryLife {
  remainingCapacity: number
  remainingTime: bigint
}
```

### ***`Memory`*** (Type)
```typescript
interface Memory {
  free: bigint
  total: bigint
  used: bigint
}
```

### ***`LoadAverage`*** (Type)
```typescript
interface LoadAverage {
  one: number
  five: number
  fifteen: number
}
```

### ***`CPULoad`*** (Type)
```typescript
interface CPULoad {
  user: number
  nice: number
  system: number
  interrupt: number
  idle: number
}
```

### ***`ConvertResult`*** (Type)
```typescript
interface ConvertResult {
  value: string
  unit: string
}
```

### ***`ConvertOption`*** (Type)
```typescript
interface ConvertOption {
  advance: Advance
  /** Decimal point */
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


## How to Develop

### Development requirements

- Install the latest `Rust`
- Install `Node.js@10+` which fully supported `Node-API`
- Install `yarn@1.x`

### Test in local

- yarn
- yarn build
- yarn test

## Release Notes
SEE [CHANGELOG](./CHANGELOG.md)

## Thanks
[systemstat](https://github.com/unrelentingtech/systemstat)

[napi-rs](https://github.com/napi-rs/napi-rs)
