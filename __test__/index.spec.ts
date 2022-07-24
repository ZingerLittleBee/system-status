import test from 'ava'

import { Advance, format, Stat } from '../index'

test('Stat', (t) => {
  const ostat = new Stat()
  const mount = ostat.mountAt('/dev')
  t.log(mount.total)
  t.log(format(mount.total, { advance: Advance.Binary, precision: 2 }))

  // const cpuLoadAggregate = ostat.cpuLoadAggregate()
  // Object.values(cpuLoadAggregate)?.forEach((value) => {
  //   t.true(value >= 0)
  // })
  // ostat.socketStats()

  // ostat.loadAverage()
  // ostat.swap()
  // // ostat.blockDeviceStatistics()
  // ostat.networks().forEach((n) => {
  //   t.log(`name: ${n.name}`)
  //   n.addrs.forEach((addr) => t.log(`addr: ${addr.addr} ${addr.addrType} ${addr.netmask}`))
  // })
  // t.log(ostat.memory())
  // t.log(ostat.uptime())
  // t.log(ostat.bootTime())
  t.pass()
})
