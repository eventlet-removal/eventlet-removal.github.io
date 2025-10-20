---
title: "Eventlet Migration Progress - 2025-10-20"
date: 2025-10-20T09:16:18.937438503+00:00
layout: analysis
type: report
overall_progress: 10.2
total_projects: 122
fully_migrated: 14
in_progress: 20
---

# OpenStack Eventlet Migration Analysis

**Analysis Date:** 2025-10-20 09:16 UTC

**Overall Progress:** 10.2%

## Summary Statistics

- **Total Projects:** 122
- **Fully Migrated:** 14 (11.5%)
- **In Progress:** 20 (16.4%)
- **Stalled:** 60 (49.2%)
- **Total Usages Removed:** 239

## Project Data

```json
[
  {
    "project_name": "puppet-neutron",
    "baseline_usages": 4,
    "current_usages": 0,
    "usages_removed": 4,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -20.0
  },
  {
    "project_name": "glance_store",
    "baseline_usages": 12,
    "current_usages": 9,
    "usages_removed": 3,
    "progress_percentage": 25.0,
    "migration_status": "InProgress",
    "complexity_change": -7.0
  },
  {
    "project_name": "kolla-ansible",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "python-ironicclient",
    "baseline_usages": 4,
    "current_usages": 4,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "barbican",
    "baseline_usages": 9,
    "current_usages": 0,
    "usages_removed": 9,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -36.0
  },
  {
    "project_name": "puppet-nova",
    "baseline_usages": 0,
    "current_usages": 1,
    "usages_removed": -1,
    "progress_percentage": -100.0,
    "migration_status": "New",
    "complexity_change": 5.0
  },
  {
    "project_name": "openstack-ansible",
    "baseline_usages": 0,
    "current_usages": 3,
    "usages_removed": -3,
    "progress_percentage": -100.0,
    "migration_status": "New",
    "complexity_change": 15.0
  },
  {
    "project_name": "tacker",
    "baseline_usages": 87,
    "current_usages": 87,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "neutron-fwaas",
    "baseline_usages": 4,
    "current_usages": 4,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "neutron-tempest-plugin",
    "baseline_usages": 5,
    "current_usages": 0,
    "usages_removed": 5,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -19.0
  },
  {
    "project_name": "puppet-openstacklib",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "swift",
    "baseline_usages": 311,
    "current_usages": 314,
    "usages_removed": -3,
    "progress_percentage": -0.964630225080386,
    "migration_status": "Regressed",
    "complexity_change": 36.0
  },
  {
    "project_name": "oslo.reports",
    "baseline_usages": 5,
    "current_usages": 5,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "osops",
    "baseline_usages": 15,
    "current_usages": 15,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "openstack-helm",
    "baseline_usages": 36,
    "current_usages": 46,
    "usages_removed": -10,
    "progress_percentage": -27.77777777777778,
    "migration_status": "Regressed",
    "complexity_change": 58.0
  },
  {
    "project_name": "blazar",
    "baseline_usages": 13,
    "current_usages": 13,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "ovn-bgp-agent",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "cinder-specs",
    "baseline_usages": 10,
    "current_usages": 10,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "python-zaqarclient",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "ceilometer",
    "baseline_usages": 6,
    "current_usages": 5,
    "usages_removed": 1,
    "progress_percentage": 16.666666666666664,
    "migration_status": "InProgress",
    "complexity_change": -5.0
  },
  {
    "project_name": "python-manilaclient",
    "baseline_usages": 1,
    "current_usages": 0,
    "usages_removed": 1,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -1.0
  },
  {
    "project_name": "puppet-manila",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "mistral",
    "baseline_usages": 40,
    "current_usages": 10,
    "usages_removed": 30,
    "progress_percentage": 75.0,
    "migration_status": "InProgress",
    "complexity_change": -81.0
  },
  {
    "project_name": "monasca-api",
    "baseline_usages": 5,
    "current_usages": 0,
    "usages_removed": 5,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -25.0
  },
  {
    "project_name": "rpm-packaging",
    "baseline_usages": 59,
    "current_usages": 54,
    "usages_removed": 5,
    "progress_percentage": 8.47457627118644,
    "migration_status": "InProgress",
    "complexity_change": -25.0
  },
  {
    "project_name": "oslo.db",
    "baseline_usages": 9,
    "current_usages": 8,
    "usages_removed": 1,
    "progress_percentage": 11.11111111111111,
    "migration_status": "InProgress",
    "complexity_change": -5.0
  },
  {
    "project_name": "mistral-lib",
    "baseline_usages": 4,
    "current_usages": 0,
    "usages_removed": 4,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -10.0
  },
  {
    "project_name": "oslo.config",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "heat-specs",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "python-glanceclient",
    "baseline_usages": 4,
    "current_usages": 4,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "ironic-python-agent",
    "baseline_usages": 7,
    "current_usages": 5,
    "usages_removed": 2,
    "progress_percentage": 28.57142857142857,
    "migration_status": "InProgress",
    "complexity_change": 2.0
  },
  {
    "project_name": "etcd3gw",
    "baseline_usages": 3,
    "current_usages": 3,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "rally-openstack",
    "baseline_usages": 0,
    "current_usages": 1,
    "usages_removed": -1,
    "progress_percentage": -100.0,
    "migration_status": "New",
    "complexity_change": 5.0
  },
  {
    "project_name": "trove",
    "baseline_usages": 34,
    "current_usages": 28,
    "usages_removed": 6,
    "progress_percentage": 17.647058823529413,
    "migration_status": "InProgress",
    "complexity_change": -30.0
  },
  {
    "project_name": "masakari-monitors",
    "baseline_usages": 27,
    "current_usages": 29,
    "usages_removed": -2,
    "progress_percentage": -7.4074074074074066,
    "migration_status": "Regressed",
    "complexity_change": 6.0
  },
  {
    "project_name": "glance-specs",
    "baseline_usages": 11,
    "current_usages": 11,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "puppet-barbican",
    "baseline_usages": 5,
    "current_usages": 1,
    "usages_removed": 4,
    "progress_percentage": 80.0,
    "migration_status": "InProgress",
    "complexity_change": -20.0
  },
  {
    "project_name": "octavia-tempest-plugin",
    "baseline_usages": 7,
    "current_usages": 7,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "cloudkitty-specs",
    "baseline_usages": 4,
    "current_usages": 4,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "python-swiftclient",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "oslo.log",
    "baseline_usages": 28,
    "current_usages": 56,
    "usages_removed": -28,
    "progress_percentage": -100.0,
    "migration_status": "Regressed",
    "complexity_change": 122.0
  },
  {
    "project_name": "tooz",
    "baseline_usages": 4,
    "current_usages": 7,
    "usages_removed": -3,
    "progress_percentage": -75.0,
    "migration_status": "Regressed",
    "complexity_change": 15.0
  },
  {
    "project_name": "oslo.utils",
    "baseline_usages": 20,
    "current_usages": 22,
    "usages_removed": -2,
    "progress_percentage": -10.0,
    "migration_status": "Regressed",
    "complexity_change": 10.0
  },
  {
    "project_name": "monasca-agent",
    "baseline_usages": 5,
    "current_usages": 0,
    "usages_removed": 5,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -17.0
  },
  {
    "project_name": "requirements",
    "baseline_usages": 2,
    "current_usages": 2,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "governance",
    "baseline_usages": 95,
    "current_usages": 18,
    "usages_removed": 77,
    "progress_percentage": 81.05263157894737,
    "migration_status": "InProgress",
    "complexity_change": -393.0
  },
  {
    "project_name": "charm-manila",
    "baseline_usages": 2,
    "current_usages": 2,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "oslo.privsep",
    "baseline_usages": 18,
    "current_usages": 20,
    "usages_removed": -2,
    "progress_percentage": -11.11111111111111,
    "migration_status": "Regressed",
    "complexity_change": 10.0
  },
  {
    "project_name": "mistral-extra",
    "baseline_usages": 3,
    "current_usages": 0,
    "usages_removed": 3,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -9.0
  },
  {
    "project_name": "election",
    "baseline_usages": 5,
    "current_usages": 19,
    "usages_removed": -14,
    "progress_percentage": -280.0,
    "migration_status": "Regressed",
    "complexity_change": 70.0
  },
  {
    "project_name": "aodh",
    "baseline_usages": 5,
    "current_usages": 5,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "monasca-events-api",
    "baseline_usages": 3,
    "current_usages": 0,
    "usages_removed": 3,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -15.0
  },
  {
    "project_name": "devstack",
    "baseline_usages": 1,
    "current_usages": 2,
    "usages_removed": -1,
    "progress_percentage": -100.0,
    "migration_status": "Regressed",
    "complexity_change": 5.0
  },
  {
    "project_name": "watcher",
    "baseline_usages": 13,
    "current_usages": 31,
    "usages_removed": -18,
    "progress_percentage": -138.46153846153845,
    "migration_status": "Regressed",
    "complexity_change": 80.0
  },
  {
    "project_name": "octavia-lib",
    "baseline_usages": 8,
    "current_usages": 8,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "networking-sfc",
    "baseline_usages": 3,
    "current_usages": 9,
    "usages_removed": -6,
    "progress_percentage": -200.0,
    "migration_status": "Regressed",
    "complexity_change": 34.0
  },
  {
    "project_name": "oslo.cache",
    "baseline_usages": 7,
    "current_usages": 11,
    "usages_removed": -4,
    "progress_percentage": -57.14285714285714,
    "migration_status": "Regressed",
    "complexity_change": 20.0
  },
  {
    "project_name": "neutron-dynamic-routing",
    "baseline_usages": 4,
    "current_usages": 4,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": -3.0
  },
  {
    "project_name": "auto-scaling-sig",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "oslo.rootwrap",
    "baseline_usages": 6,
    "current_usages": 7,
    "usages_removed": -1,
    "progress_percentage": -16.666666666666664,
    "migration_status": "Regressed",
    "complexity_change": 5.0
  },
  {
    "project_name": "heat",
    "baseline_usages": 63,
    "current_usages": 2,
    "usages_removed": 61,
    "progress_percentage": 96.82539682539682,
    "migration_status": "InProgress",
    "complexity_change": -227.0
  },
  {
    "project_name": "neutron-lib",
    "baseline_usages": 8,
    "current_usages": 9,
    "usages_removed": -1,
    "progress_percentage": -12.5,
    "migration_status": "Regressed",
    "complexity_change": 5.0
  },
  {
    "project_name": "python-keystoneclient",
    "baseline_usages": 5,
    "current_usages": 5,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "ironic-inspector",
    "baseline_usages": 17,
    "current_usages": 0,
    "usages_removed": 17,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -69.0
  },
  {
    "project_name": "keystone-specs",
    "baseline_usages": 3,
    "current_usages": 3,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "puppet-keystone",
    "baseline_usages": 6,
    "current_usages": 6,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "cloudkitty",
    "baseline_usages": 3,
    "current_usages": 3,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "networking-generic-switch",
    "baseline_usages": 7,
    "current_usages": 2,
    "usages_removed": 5,
    "progress_percentage": 71.42857142857143,
    "migration_status": "InProgress",
    "complexity_change": -18.0
  },
  {
    "project_name": "openstack-ansible-os_neutron",
    "baseline_usages": 2,
    "current_usages": 3,
    "usages_removed": -1,
    "progress_percentage": -50.0,
    "migration_status": "Regressed",
    "complexity_change": 5.0
  },
  {
    "project_name": "puppet-ceilometer",
    "baseline_usages": 2,
    "current_usages": 2,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "keystonemiddleware",
    "baseline_usages": 11,
    "current_usages": 11,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "releases",
    "baseline_usages": 1,
    "current_usages": 4,
    "usages_removed": -3,
    "progress_percentage": -300.0,
    "migration_status": "Regressed",
    "complexity_change": 15.0
  },
  {
    "project_name": "networking-baremetal",
    "baseline_usages": 3,
    "current_usages": 2,
    "usages_removed": 1,
    "progress_percentage": 33.33333333333333,
    "migration_status": "InProgress",
    "complexity_change": -1.0
  },
  {
    "project_name": "taskflow",
    "baseline_usages": 32,
    "current_usages": 34,
    "usages_removed": -2,
    "progress_percentage": -6.25,
    "migration_status": "Regressed",
    "complexity_change": 10.0
  },
  {
    "project_name": "nova",
    "baseline_usages": 139,
    "current_usages": 110,
    "usages_removed": 29,
    "progress_percentage": 20.863309352517987,
    "migration_status": "InProgress",
    "complexity_change": -94.0
  },
  {
    "project_name": "glance",
    "baseline_usages": 112,
    "current_usages": 101,
    "usages_removed": 11,
    "progress_percentage": 9.821428571428571,
    "migration_status": "InProgress",
    "complexity_change": -39.0
  },
  {
    "project_name": "neutron-vpnaas",
    "baseline_usages": 6,
    "current_usages": 0,
    "usages_removed": 6,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -16.0
  },
  {
    "project_name": "bifrost",
    "baseline_usages": 0,
    "current_usages": 1,
    "usages_removed": -1,
    "progress_percentage": -100.0,
    "migration_status": "New",
    "complexity_change": 5.0
  },
  {
    "project_name": "nova-specs",
    "baseline_usages": 4,
    "current_usages": 4,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "oslo.messaging",
    "baseline_usages": 66,
    "current_usages": 64,
    "usages_removed": 2,
    "progress_percentage": 3.0303030303030303,
    "migration_status": "InProgress",
    "complexity_change": -2.0
  },
  {
    "project_name": "python-cinderclient",
    "baseline_usages": 1,
    "current_usages": 0,
    "usages_removed": 1,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -1.0
  },
  {
    "project_name": "ossa",
    "baseline_usages": 2,
    "current_usages": 2,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "keystone",
    "baseline_usages": 20,
    "current_usages": 20,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "rally",
    "baseline_usages": 3,
    "current_usages": 4,
    "usages_removed": -1,
    "progress_percentage": -33.33333333333333,
    "migration_status": "Regressed",
    "complexity_change": 5.0
  },
  {
    "project_name": "cyborg",
    "baseline_usages": 4,
    "current_usages": 4,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "ironic",
    "baseline_usages": 33,
    "current_usages": 21,
    "usages_removed": 12,
    "progress_percentage": 36.36363636363637,
    "migration_status": "InProgress",
    "complexity_change": -18.0
  },
  {
    "project_name": "ironic-specs",
    "baseline_usages": 1,
    "current_usages": 3,
    "usages_removed": -2,
    "progress_percentage": -200.0,
    "migration_status": "Regressed",
    "complexity_change": 10.0
  },
  {
    "project_name": "storlets",
    "baseline_usages": 7,
    "current_usages": 7,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "designate-specs",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "octavia",
    "baseline_usages": 11,
    "current_usages": 11,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "os-ken",
    "baseline_usages": 37,
    "current_usages": 37,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": -3.0
  },
  {
    "project_name": "cinder",
    "baseline_usages": 167,
    "current_usages": 167,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "oslo.service",
    "baseline_usages": 66,
    "current_usages": 108,
    "usages_removed": -42,
    "progress_percentage": -63.63636363636363,
    "migration_status": "Regressed",
    "complexity_change": 206.0
  },
  {
    "project_name": "neutron",
    "baseline_usages": 174,
    "current_usages": 58,
    "usages_removed": 116,
    "progress_percentage": 66.66666666666666,
    "migration_status": "InProgress",
    "complexity_change": -461.0
  },
  {
    "project_name": "python-mistralclient",
    "baseline_usages": 1,
    "current_usages": 0,
    "usages_removed": 1,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -5.0
  },
  {
    "project_name": "neutron-specs",
    "baseline_usages": 7,
    "current_usages": 7,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "oslo-specs",
    "baseline_usages": 51,
    "current_usages": 87,
    "usages_removed": -36,
    "progress_percentage": -70.58823529411765,
    "migration_status": "Regressed",
    "complexity_change": 180.0
  },
  {
    "project_name": "openstack-ansible-ops",
    "baseline_usages": 10,
    "current_usages": 10,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "charm-manila-ganesha",
    "baseline_usages": 2,
    "current_usages": 2,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "swift-bench",
    "baseline_usages": 14,
    "current_usages": 14,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "oslo.concurrency",
    "baseline_usages": 17,
    "current_usages": 12,
    "usages_removed": 5,
    "progress_percentage": 29.411764705882355,
    "migration_status": "InProgress",
    "complexity_change": -17.0
  },
  {
    "project_name": "masakari",
    "baseline_usages": 28,
    "current_usages": 28,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "magnum",
    "baseline_usages": 8,
    "current_usages": 8,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "python-octaviaclient",
    "baseline_usages": 6,
    "current_usages": 6,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "openstack-manuals",
    "baseline_usages": 23,
    "current_usages": 34,
    "usages_removed": -11,
    "progress_percentage": -47.82608695652174,
    "migration_status": "Regressed",
    "complexity_change": 55.0
  },
  {
    "project_name": "telemetry-specs",
    "baseline_usages": 12,
    "current_usages": 12,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "zun",
    "baseline_usages": 15,
    "current_usages": 15,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "ovsdbapp",
    "baseline_usages": 7,
    "current_usages": 7,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "hacking",
    "baseline_usages": 2,
    "current_usages": 2,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "venus",
    "baseline_usages": 22,
    "current_usages": 22,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "oslo.vmware",
    "baseline_usages": 4,
    "current_usages": 4,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "upstream-institute-virtual-environment",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "futurist",
    "baseline_usages": 24,
    "current_usages": 25,
    "usages_removed": -1,
    "progress_percentage": -4.166666666666666,
    "migration_status": "Regressed",
    "complexity_change": 5.0
  },
  {
    "project_name": "kolla",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "security-doc",
    "baseline_usages": 4,
    "current_usages": 4,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "project-config",
    "baseline_usages": 3,
    "current_usages": 4,
    "usages_removed": -1,
    "progress_percentage": -33.33333333333333,
    "migration_status": "Regressed",
    "complexity_change": 5.0
  },
  {
    "project_name": "heat-templates",
    "baseline_usages": 1,
    "current_usages": 0,
    "usages_removed": 1,
    "progress_percentage": 100.0,
    "migration_status": "FullyMigrated",
    "complexity_change": -5.0
  },
  {
    "project_name": "os-brick",
    "baseline_usages": 2,
    "current_usages": 2,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "vitrage",
    "baseline_usages": 2,
    "current_usages": 2,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  },
  {
    "project_name": "designate",
    "baseline_usages": 44,
    "current_usages": 41,
    "usages_removed": 3,
    "progress_percentage": 6.8181818181818175,
    "migration_status": "InProgress",
    "complexity_change": -5.0
  },
  {
    "project_name": "manila",
    "baseline_usages": 33,
    "current_usages": 32,
    "usages_removed": 1,
    "progress_percentage": 3.0303030303030303,
    "migration_status": "InProgress",
    "complexity_change": -2.0
  },
  {
    "project_name": "python-troveclient",
    "baseline_usages": 1,
    "current_usages": 1,
    "usages_removed": 0,
    "progress_percentage": 0.0,
    "migration_status": "Stalled",
    "complexity_change": 0.0
  }
]
```
