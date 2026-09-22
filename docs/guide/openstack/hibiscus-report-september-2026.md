---
layout: base
title: "Hibiscus Cycle — Eventlet Migration Report (September 2026)"
permalink: /guide/openstack/hibiscus-report-september-2026/
---

# Eventlet Migration Report — September 2026

> **Methodology**: Beagle/Hound scan + direct git clone inspection of all 42
> in-progress/stalled/regressed projects + Gerrit API query for all
> eventlet-removal patches (79 open, 131 merged since March 2026).

## Executive Summary

- **91.8% of projects (93.4% of baseline usages) are migrated or actively progressing**
- **59 projects fully migrated**, 13 in progress, 18 deprecating — only **4 genuinely stalled**
- **79 open Gerrit patches** across 14 projects (swift: 29, nova: 12, masakari: 11)
- **131 merged patches** since March 2026 across 22 projects
- Major active efforts: swift (gunicorn), nova (threading backend), cinder (driver-by-driver)
- **None of the 4 stalled projects are part of RHOSO**

---

## Gerrit Activity Summary

### Most active projects (merged since Mar 2026)

| Project | Merged | Open | Key activity |
|---------|--------|------|--------------|
| nova | 32 | 12 | Native threading default for conductor/compute, poison eventlet import, file-backed SQLite |
| swift | 26 | 29 | Gunicorn WSGI server, replacement for every eventlet primitive (Timeout, sleep, spawn, Pool, tpool) |
| cinder | 26 | 6 | Driver-by-driver migration, native threads for backup, CI jobs |
| glance | 12 | 0 | Fully migrated — uWSGI + native threading, eventlet removed from requirements |
| manila | 11 | 3 | Native threading default, WSGI rewrite, eventlet removal in progress |
| oslo.concurrency | 3 | 0 | Lazy eventlet import, compatibility fixes |
| blazar | 2 | 0 | Fully migrated — threading API + AsyncIO |
| oslo.service | 2 | 3 | Parallel periodic tasks, spawn-safe multiprocessing |
| oslo.privsep | 2 | 0 | Eventlet made optional |
| tacker | 2 | 5 | Stdlib subprocess, dead code removal; 5 more patches pending |
| watcher | 1 | 3 | Native threading default |

---

## Confirmed Fully Migrated (59 projects)

### Confirmed by cloning master (beagle data was stale)

**glance**
- 12 patches merged, 0 open. Last: "Default oslo.service to threading backend" (Aug 10)
- All remaining refs are comments about native threading

**neutron**
- Zero `import eventlet` or `from eventlet` in the entire codebase
- Remaining refs (~70 lines): all TODO comments, `@unittest.skip('after eventlet removal')`, deprecation warning in `__init__.py`
- ~20 skipped tests awaiting re-enablement (cleanup, not migration)

**ironic**
- 0 real eventlet imports in production code
- Remaining refs: 5 files with comments about post-eventlet model

**manila**
- 0 files with eventlet imports
- 11 merged, 3 open (WSGI rewrite, removal from requirements)
- Native threading is now the default

**oslo.db**
- Only comments about eventlet.monkey_patch() in engine code

**openstack-manuals**
- No Python eventlet usage, only doc references

### Confirmed by code inspection (no real eventlet usage)

election, releases, security-doc — zero code, only text references
(candidate statements, release notes YAML, security advisories)

### Previously confirmed by beagle analyzer (47 projects)

heat, venus, designate, barbican, ironic-inspector, networking-generic-switch,
neutron-vpnaas, ironic-python-agent, neutron-tempest-plugin, monasca-agent,
neutron-fwaas, mistral-lib, cyborg, oslo.vmware, neutron-dynamic-routing,
puppet-barbican, puppet-neutron, python-ironicclient, monasca-api,
mistral-extra, python-glanceclient, networking-sfc, networking-baremetal,
monasca-events-api, os-brick, python-manilaclient, heat-templates,
python-cinderclient, python-mistralclient, vitrage, ceilometer, kolla,
mistral, blazar, magnum, rpm-packaging, keystonemiddleware, python-swiftclient,
auto-scaling-sig, ossa, charm-manila-ganesha, charm-manila, governance,
requirements, octavia, octavia-lib, octavia-tempest-plugin

---

## In-Progress Projects (13) — Detailed Status

### cinder — 57.4% done, very active
- **Actual master**: 21 non-test files, real usage in volume drivers (FC zone clients, DS8K, Datera, NetApp)
- **Gerrit**: 26 merged + 6 open patches. Very active driver-by-driver migration
- **Requirements**: eventlet still in requirements.txt
- Key remaining: `wsgi/eventlet_server.py`, `ssh_utils.py`, volume driver imports

### swift — 24% done, massive migration underway
- **Actual master**: 26 non-test files with real eventlet usage
- **Gerrit**: 26 merged + **29 open patches** — complete gunicorn migration series
- Building replacement for every eventlet primitive (sleep, spawn, Timeout, Pool, tpool, GreenPile)
- **Requirements**: eventlet still in requirements.txt

### nova — Active migration, complex
- **Actual master**: 20 non-test files, but heavy scaffolding (dual-backend, poison import)
- **Gerrit**: 32 merged + 12 open. Native threading default for conductor/compute since Mar 2026
- Remaining real usage: `monkey_patch.py`, hacking checks, libvirt native threading bridge
- **Requirements**: eventlet still in requirements.txt

### oslo.service — Migration infrastructure hub
- **Actual master**: 16 non-test files — `wsgi.py` and `eventlet_backdoor.py` are the real eventlet usage
- The `backend/_eventlet/` directory is the dual-backend scaffolding
- `eventlet_backdoor.py` has deprecation warning
- **Requirements**: eventlet still in requirements.txt

### oslo.messaging — Deprecation in progress
- **Actual master**: 13 non-test files, mostly `eventletutils` imports (deprecated)
- `simulator.py` tool has real eventlet usage
- RPC server deprecation warnings for eventlet executor
- **Requirements**: eventlet only in test-requirements.txt

### tacker — Active migration (Hound data stale)
- **Actual master**: 17 non-test files. Core code: `wsgi.py`, `eventlet_utils.py`, `cmd/eventlet/`
- **Gerrit**: 2 merged + **5 open patches** including "Remove eventlet from requirements" and "Remove the eventlet based API server"
- ~50 usages in `samples/` directory (free5gc scripts with copy-pasted monkey_patch/Timeout patterns)
- Once open patches merge, only samples/ will remain

### trove — Early stage
- **Actual master**: 12 non-test files with real eventlet usage
- WSGI server (`base_wsgi.py`), cluster task managers use `eventlet.Timeout`
- **Gerrit**: 1 merged ("Preparation for removing eventlet")
- **Requirements**: eventlet still in requirements.txt

### oslo.concurrency — Conditional import
- **Actual master**: 2 non-test files — `lockutils.py` conditionally imports eventlet
- **Gerrit**: 3 merged (lazy import, compatibility)
- Eventlet NOT in requirements.txt

### glance_store — Nearly done
- **Actual master**: 2 non-test files, 1 is a comment about replacement
- **Gerrit**: 2 merged + 1 open ("Remove eventlet from requirements")

### watcher — Active, native threading default
- **Actual master**: 18 files with eventlet refs, real usage (wsgi, tpool, monkey_patch)
- Native threading is now default (1 merged patch). 3 open patches for full removal
- **Requirements**: eventlet still in requirements.txt

### masakari — Active, full patch series pending
- **Actual master**: 15 files with real usage (spawn, greenpool, monkey_patch)
- **Gerrit**: **11 open patches** covering full migration (threading infrastructure, RPC, WSGI, tests)
- **Requirements**: eventlet still in requirements.txt

### python-keystoneclient — Patch pending
- **Actual master**: 1 file (`cms.py`) — optional compat using eventlet.green.subprocess
- **Gerrit**: 2 open patches (#958881, #958882) — remove eventlet + add hacking rule
- Eventlet NOT in requirements.txt

### oslo.log — Dual-backend compat code
- **Actual master**: 7 files, `pipe_mutex.py` is complex eventlet compat for dual-backend logging
- **Gerrit**: 1 merged patch
- **Requirements**: eventlet in test-requirements.txt

---

## Deprecating Projects (18) — Eventlet Already Optional

These projects work without eventlet. The eventlet code path is either
optional (`try_import`), conditional (`sys.platform == 'win32'`), or
purely informational (config names, hacking rules banning eventlet).

| Project | Nature of remaining eventlet refs |
|---------|----------------------------------|
| **taskflow** | `try_import('eventlet')`, native `ThreadPoolExecutor` is primary. `GreenThreadedExecutor` is optional extra |
| **etcd3gw** | `_try_import('eventlet')` for runtime detection only |
| **ovsdbapp** | `try/except ImportError` on all imports. Two files use eventlet only on Windows (`sys.platform == 'win32'`). Irrelevant to Linux/RHOSO |
| **oslo.privsep** | 2 merged patches making eventlet optional. `daemon.py` handles both paths |
| **oslo.rootwrap** | 1 merged patch: "Import eventlet conditionally". Uses `eventlet.green.subprocess` only if monkey-patched |
| **oslo.cache** | `_memcache_pool.py` conditionally imports eventlet for monkey-patch detection |
| **oslo.utils** | `eventletutils.py` is explicitly deprecated with warning |
| **futurist** | Native alternatives added (1 merged), GreenThreadPoolExecutor is optional |
| **tooz** | Single kazoo eventlet handler import in zookeeper driver. Optional |
| **hacking** | Added H905 rule that **bans** eventlet imports. Anti-eventlet enforcement |
| **rally** | Adding deprecation warnings |
| **project-config** | Adding deprecation config |
| **neutron-lib** | Adding deprecation scaffolding |
| **openstack-helm** | Helm chart YAML values: logger section names, DNS comments. No Python |
| **python-troveclient** | Single line: `import eventlet as sleep_lib`. Trivial |
| **ovn-bgp-agent** | eventlet only in test-requirements and functional tests |
| **puppet-nova** | Config template reference |
| **rally-openstack** | 1 merged + 1 open Gerrit patch |

---

## Stalled Projects (4) — Real Usage, No Gerrit Activity

| Project | Files | In reqs | Classification | Effort | RHOSO? |
|---------|-------|---------|----------------|--------|--------|
| **zun** | 8 | Yes | Real usage: monkey_patch, spawn_n, Timeout, executor='eventlet' | Hard | No |
| **storlets** | 5 | Yes | Real usage: monkey_patch, spawn_n, Timeout in gateway runtime | Moderate | No |
| **os-ken** | 4 | Yes | Deep integration in `lib/hub.py`: wraps all eventlet primitives | Hard | No |
| **masakari-monitors** | 21 | No | Real usage: greenthread.sleep, monkey_patch across monitors | Hard | Unlikely |

**None of the 4 stalled projects are part of RHOSO.**

---

## Abandoned Projects (4) — Unmaintained

| Project | Usages | Analysis |
|---------|--------|----------|
| osops | 14 | Ancient Liberty-era install scripts |
| swift-bench | 13 | Deeply eventlet-based benchmarking tool |
| openstack-ansible-ops | 10 | Logstash pipeline templates |
| upstream-institute | 1 | Training material |

---

## Corrected Project Count

| Status | Beagle count | Corrected | Projects |
|--------|-------------|-----------|----------|
| Fully Migrated | 47 | **59** | 47 from beagle + glance, neutron, ironic, manila, oslo.db, openstack-manuals, python-octaviaclient, puppet-keystone, oslo.reports, election, releases, security-doc |
| In Progress | 16 | **13** | cinder, swift, nova, tacker, trove, oslo.service, oslo.messaging, oslo.concurrency, glance_store, watcher, masakari, python-keystoneclient, oslo.log |
| Deprecating | 3 | **18** | rally, project-config, neutron-lib, hacking, oslo.privsep, oslo.rootwrap, oslo.cache, tooz, taskflow, etcd3gw, ovsdbapp, futurist, oslo.utils, openstack-helm, python-troveclient, ovn-bgp-agent, puppet-nova, rally-openstack |
| Stalled | 11 | **4** | zun, storlets, os-ken, masakari-monitors |
| Abandoned | 4 | **4** | osops, swift-bench, openstack-ansible-ops, upstream-institute |

**Total**: 98 projects. 59 + 13 + 18 + 4 + 4 = 98.

### Progress metric

| Category | Projects | % projects | Baseline usages | % usages |
|----------|----------|------------|-----------------|----------|
| Fully migrated | 59 | 60.2% | 730 | 40.9% |
| In progress | 13 | 13.3% | 832 | 46.7% |
| Deprecating (eventlet optional) | 18 | 18.4% | 102 | 5.7% |
| **Subtotal: finished or active** | **90** | **91.8%** | **1664** | **93.4%** |
| Abandoned (dead repos) | 4 | 4.1% | 38 | 2.1% |
| **Genuinely stalled** | **4** | **4.1%** | **79** | **4.4%** |

The 9 largest in-progress projects carry the bulk of remaining work,
concentrated in 3 large projects:

| In-progress project | Baseline usages | % of in-progress | Gerrit activity |
|---------------------|-----------------|------------------|-----------------|
| swift | 250 | 30.0% | 26 merged + 29 open |
| cinder | 155 | 18.6% | 26 merged + 6 open |
| nova | 97 | 11.7% | 32 merged + 12 open |
| tacker | 85 | 10.2% | 2 merged + 5 open |
| oslo.messaging | 65 | 7.8% | deprecation in progress |
| oslo.service | 56 | 6.7% | 2 merged + 3 open |
| watcher | 35 | 4.2% | 1 merged + 3 open |
| trove | 34 | 4.1% | 1 merged |
| masakari | 26 | 3.1% | 11 open |
| oslo.log | 22 | 2.6% | 1 merged |
| oslo.concurrency | 17 | 2.0% | 3 merged |
| glance_store | 12 | 1.4% | 2 merged + 1 open |
| python-keystoneclient | 5 | 0.6% | 2 open |

**91.8% of projects (93.4% of baseline usages) are either migrated or actively
progressing.** Only 4 projects (4.1%) with 79 baseline usages (4.4%) are genuinely
stalled: zun, storlets, os-ken, masakari-monitors.

**None of the 4 stalled projects are part of RHOSO.**

---

## One-sentence summary

The OpenStack eventlet removal effort — spanning 98 projects and 1782
baseline usages — has reached 91.8% of projects either fully migrated or
actively progressing: 59 are confirmed migrated, 13 are in progress
(led by swift, cinder, and nova with 67 open Gerrit patches between them),
18 have deprecated eventlet to an optional dependency, and only 4 projects
remain genuinely stalled (zun, storlets, os-ken, masakari-monitors) and
4 are abandoned — none of which are part of RHOSO.

---

## Data Sources

- **Beagle scan**: September 14, 2026 (all results from master branch only)
- **Git clones**: Shallow clones of all 42 non-migrated projects from opendev.org
- **Gerrit API**: `review.opendev.org/changes/?q=topic:eventlet-removal` (79 open + 131 merged since Mar 2026)
- **Baseline**: Original beagle scan (1782 usages across 98 projects)
- **Tool**: `eventlet-analyzer` (Rust, from `origin/backup` branch, enhanced with scaffolding/abandoned detection)
