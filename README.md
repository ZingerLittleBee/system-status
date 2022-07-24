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
- [Goods](#goods)
- [Documentation](#documentation)
  - [`mounts`](#mounts)
  - [`mountAt`](#mountat)
  - [`networks`](#networks)
  - [`memory`](#memory)
  - [`swap`](#swap)
  - [`loadAverage`](#loadaverage)
  - [`uptime`](#uptime)
  - [`bootTime`](#boottime)
  - [`cpuLoadAggregate`](#cpuloadaggregate)
  - [`cpuTemp`](#cputemp)
  - [`socketStats`](#socketstats)
  - [`format`](#format)
  - [`FileSystem` (Type)](#filesystem-type)
  - [`Network` (Type)](#network-type)
  - [`Memory` (Type)](#memory-type)
  - [`LoadAverage` (Type)](#loadaverage-type)
  - [`CPULoad` (Type)](#cpuload-type)
  - [`ConvertResult` (Type)](#convertresult-type)
  - [`ConvertOption` (Type)](#convertoption-type)
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

## Goods
Class: `Stat`
  - [`mounts`](#mounts)
  - [`mountAt`](#mountat)
  - [`networks`](#network)
  - [`memory`](#memory)
  - [`swap`](#swap)
  - [`loadAverage`](#loadaverage)
  - [`uptime`](#uptime)
  - [`bootTime`](#boottime)
  - [`cpuLoadAggregate`](#cpuloadaggregate)
  - [`cpuTemp`](#cputemp)
  - [`socketStats`](#socketstats)

Function: [`format`](#format)

Type:
  - [`FileSystem`](#filesystem-type)
  - [`Network`](#network-type)
    - `NetworkAddrs`
    - `AddrType`
  - [`Memory`](#memory-type)
  - [`LoadAverage`](#loadaverage-type)
  - [`CPULoad`](#cpuload-type)
  - [`ConvertResult`](#convertresult-type)
  - [`ConvertOption`](#convertoption-type)
    - `Advance`


## Documentation
Create `Stat` class instance in first
```typescript
const stat = new Stat()
```
### `mounts`
Get filesystem mount information.
```typescript
mounts(): Array<FileSystem>
```

### `mountAt`
Get a filesystem mount information for the filesystem at a given path.
```typescript
mountAt(at: string): FileSystem
```

### `networks`
Get network intefrace information.
```typescript
networks(): Array<Network>
```

### `memory`
Get memory information.
```typescript
memory(): Memory
```

### `swap`
Get swap memory information.
```typescript
swap(): Memory
```

### `loadAverage`
Get load average.
```typescript
loadAverage(): LoadAverage
```

### `uptime`
Get the system uptime.
```typescript
uptime(): bigint
```

### `bootTime`
Get the system boot time.
```typescript
bootTime(): Date
```

### `cpuLoadAggregate`
Get CPU load statistics, average over all CPUs (cores).
```typescript
cpuLoadAggregate(): CPULoad
```

### `cpuTemp`
Get the current CPU temperature in degrees Celsius.
Depending on the platform, this might be core 0, package, etc.
```typescript
cpuTemp(): number
```

### `socketStats`
Get information about the number of sockets in use
```typescript
socketStats(): SocketStats
```

### `format`
Format the source by ConvertOption

The default of ConvertOption is taking ***1024*** as the advancement, retain ***one decimal*** place
```typescript
function format(source: bigint, option?: ConvertOption | undefined | null): ConvertResult
```

### `FileSystem` (Type)
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

### `Network` (Type)
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

### `Memory` (Type)
```typescript
interface Memory {
  free: bigint
  total: bigint
  used: bigint
}
```

### `LoadAverage` (Type)
```typescript
interface LoadAverage {
  one: number
  five: number
  fifteen: number
}
```

### `CPULoad` (Type)
```typescript
interface CPULoad {
  user: number
  nice: number
  system: number
  interrupt: number
  idle: number
  iowait: number
}
```

### `ConvertResult` (Type)
```typescript
interface ConvertResult {
  value: string
  unit: string
}
```

### `ConvertOption` (Type)
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
