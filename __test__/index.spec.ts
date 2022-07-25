import test from 'ava'

import { format, Stat } from '../index'

const ostat = new Stat()

test('mounts', (t) => {
  t.truthy(ostat.mounts().length > 0)
})

test('mountAt', (t) => {
  t.notThrows(() => ostat.mountAt('/'))
})

test('networks', (t) => {
  t.true(ostat.networks().length > 0)
})

test('memory', (t) => {
  const mem = ostat.memory()
  t.true(mem.free > 0)
  t.true(mem.used > 0)
  t.true(mem.total > 0)
})

test('swap', (t) => {
  const swap = ostat.swap()
  t.true(swap.free >= 0)
  t.true(swap.used >= 0)
  t.true(swap.total >= 0)
})

test('loadAverage', (t) => {
  const loadAverage = ostat.loadAverage()
  t.true(loadAverage.one >= 0)
  t.true(loadAverage.five >= 0)
  t.true(loadAverage.fifteen >= 0)
})

test('uptime', (t) => {
  const uptime = ostat.uptime()
  t.true(uptime > 0)
})

test('bootTime', (t) => {
  const bootTime = ostat.bootTime()
  t.truthy(bootTime)
})

test('cpuLoadAggregate', (t) => {
  const cpuLoadAggregate = ostat.cpuLoadAggregate()
  t.true(cpuLoadAggregate.user >= 0)
  t.true(cpuLoadAggregate.nice >= 0)
  t.true(cpuLoadAggregate.system >= 0)
  t.true(cpuLoadAggregate.interrupt >= 0)
  t.true(cpuLoadAggregate.idle >= 0)
  t.true(cpuLoadAggregate.iowait >= 0)
})

test('cpuTemp', (t) => {
  let cpuTemp = 0
  try {
    cpuTemp = ostat.cpuTemp()
  } catch {
    t.log('not support cpu temp')
    t.pass()
  }
  t.true(cpuTemp >= 0)
})

test('socketStats', (t) => {
  const socketStats = ostat.socketStats()
  t.true(socketStats.tcpSocketsInUse >= 0)
  t.true(socketStats.tcpSocketsOrphaned >= 0)
  t.true(socketStats.udpSocketsInUse >= 0)
  t.true(socketStats.tcp6SocketsInUse >= 0)
  t.true(socketStats.udp6SocketsInUse >= 0)
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
  t.true(ostat.blockDeviceStatistics().length > 0)
})

test('batteryLife', (t) => {
  let batteryLife
  try {
    batteryLife = ostat.batteryLife()
    t.true(batteryLife.remainingCapacity >= 0)
    t.true(batteryLife.remainingTime >= 0)
  } catch {
    t.log('not support battery life')
    t.pass()
  }
})

test('isAcPower', (t) => {
  let isAcPower
  try {
    isAcPower = ostat.isAcPower()
    t.is(typeof isAcPower === 'boolean', true)
  } catch {
    t.log('not support AC power')
    t.pass()
  }
})
