import test from 'ava'

import { Stat } from '../index'

test('Stat', (t) => {
  const ostat = new Stat()
  ostat.cpuLoad()
  ostat.socketStats()
  ostat.uptime()
  ostat.loadAverage()
  ostat.swap()
  t.pass()
})
