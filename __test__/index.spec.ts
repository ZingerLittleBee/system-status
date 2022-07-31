import test from 'ava'

import { format, Stat } from '../index'

const stat = new Stat()

test('mounts', (t) => {
  try {
    const mounts = stat.mounts()
    t.true(mounts.length > 0)
  } catch {
    t.log('not support mounts')
    t.pass()
  }
})

test('mountAt', (t) => {
  try {
    const mountAt = stat.mountAt('/')
    t.true(mountAt?.files >= 0)
    t.true(mountAt.filesTotal >= 0)
    t.true(mountAt.filesAvail >= 0)
    t.true(mountAt.free >= 0)
    t.true(mountAt.avail >= 0)
    t.true(mountAt.total >= 0)
    t.true(mountAt.nameMax >= 0)
    t.truthy(mountAt?.fsType)
    t.truthy(mountAt?.fsMountedFrom)
    t.truthy(mountAt?.fsMountedOn)
  } catch {
    t.log('not support mountAt /')
    t.pass()
  }
})

test('networks', (t) => {
  try {
    const net = stat.networks()
    t.true(net.length >= 0)
  } catch {
    t.log('not support networks')
    t.pass()
  }
})

test('memory', (t) => {
  try {
    const mem = stat.memory()
    t.true(mem.free > 0)
    t.true(mem.used > 0)
    t.true(mem.total > 0)
  } catch {
    t.log('not support memory')
    t.pass()
  }
})

test('swap', (t) => {
  try {
    const swap = stat.swap()
    t.true(swap.free >= 0)
    t.true(swap.used >= 0)
    t.true(swap.total >= 0)
  } catch {
    t.log('not support swap')
    t.pass()
  }
})

test('loadAverage', (t) => {
  try {
    const loadAverage = stat.loadAverage()
    t.true(loadAverage.one >= 0)
    t.true(loadAverage.five >= 0)
    t.true(loadAverage.fifteen >= 0)
  } catch {
    t.log('not support loadAverage')
    t.pass()
  }
})

test('uptime', (t) => {
  try {
    const uptime = stat.uptime()
    t.true(uptime > 0)
  } catch {
    t.log('not support uptime')
    t.pass()
  }
})

test('bootTime', (t) => {
  try {
    const bootTime = stat.bootTime()
    t.truthy(bootTime)
  } catch {
    t.log('not support bootTime')
    t.pass()
  }
})

test('cpuLoadAggregate', (t) => {
  try {
    const cpuLoadAggregate = stat.cpuLoadAggregate()
    t.true(cpuLoadAggregate.user >= 0)
    t.true(cpuLoadAggregate.nice >= 0)
    t.true(cpuLoadAggregate.system >= 0)
    t.true(cpuLoadAggregate.interrupt >= 0)
    t.true(cpuLoadAggregate.idle >= 0)
  } catch {
    t.log('not support cpuLoadAggregate')
    t.pass()
  }
})

test('cpuTemp', (t) => {
  try {
    const cpuTemp = stat.cpuTemp()
    t.true(cpuTemp >= 0)
  } catch {
    t.log('not support cpu temp')
    t.pass()
  }
})

test('socketStats', (t) => {
  try {
    const socketStats = stat.socketStats()
    t.true(socketStats.tcpSocketsInUse >= 0)
    t.true(socketStats.tcpSocketsOrphaned >= 0)
    t.true(socketStats.udpSocketsInUse >= 0)
    t.true(socketStats.tcp6SocketsInUse >= 0)
    t.true(socketStats.udp6SocketsInUse >= 0)
  } catch {
    t.log('not support socket stats')
    t.pass()
  }
})

test('format', (t) => {
  const f = format(1024n, { precision: 2, advance: 1024 })
  t.true(f.value === '1.00')
  t.true(f.unit === 'KIB')

  const o = format(1000000n, { precision: 2, advance: 1000 })
  t.true(o.value === '1.00')
  t.true(o.unit === 'MB')

  const r = format(1048576n)
  t.true(r.value === '1.0')
  t.true(r.unit === 'MIB')
})

test('blockDeviceStatistics', (t) => {
  try {
    const blockDeviceStatistics = stat.blockDeviceStatistics()
    t.true(blockDeviceStatistics.length > 0)
  } catch {
    t.log('not support block device statistics')
    t.pass()
  }
})

test('batteryLife', (t) => {
  try {
    const batteryLife = stat.batteryLife()
    t.true(batteryLife.remainingCapacity >= 0)
    t.true(batteryLife.remainingTime >= 0)
  } catch {
    t.log('not support battery life')
    t.pass()
  }
})

test('isAcPower', (t) => {
  try {
    const isAcPower = stat.isAcPower()
    t.is(typeof isAcPower === 'boolean', true)
  } catch {
    t.log('not support AC power')
    t.pass()
  }
})

test('networkStats', (t) => {
  try {
    const networkStats = stat.networkStats('eth0')
    t.true(networkStats.rxBytes >= 0)
    t.true(networkStats.txBytes >= 0)
    t.true(networkStats.rxPackets >= 0)
    t.true(networkStats.txPackets >= 0)
    t.true(networkStats.rxErrors >= 0)
    t.true(networkStats.txErrors >= 0)
  } catch {
    t.log('not support network stats or eth0 not exist')
    t.pass()
  }
})
