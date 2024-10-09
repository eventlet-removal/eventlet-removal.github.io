# Analysis for Team: OpenStack-Helm

## Project: openstack-helm
The issue is with the `logger` configuration in various OpenStack projects. The `eventlet` package is used as a WSGI server, but its `logger` module is not properly configured.

Specifically, the `qualname` property in the `values.yaml` files for each project is set to `"eventlet.wsgi.server"`, which is correct. However, the `logger` configuration is not properly defined, leading to a missing logger event handler.

To fix this issue, you need to add the following configuration to the `logger` section in each project's `values.yaml` file:
```yaml
logger:
  - name: eventlet.wsgi.server
    level: INFO
```
This sets the log level for the `eventlet.wsgi.server` logger to `INFO`, which should fix the issue.

Here is an example of what the updated `logger` section might look like in a project's `values.yaml` file:
```yaml
logger:
  - name: eventlet.wsgi.server
    level: INFO
```
Note that you may need to adjust the log level or add additional logger configurations depending on your specific use case.

Occurrences Found:
- https://opendev.org/openstack/openstack-helm/src/branch/master/aodh/values.yaml#n506 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/aodh/values.yaml#n509 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/barbican/values.yaml#n462 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/barbican/values.yaml#n465 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/cinder/values.yaml#n884 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/cinder/values.yaml#n887 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/designate/values.yaml#n501 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/designate/values.yaml#n504 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/doc/source/specs/nginx-sidecar.rst#n16 : performance impact, for example, eventlet.
- https://opendev.org/openstack/openstack-helm/src/branch/master/glance/values.yaml#n344 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/glance/values.yaml#n347 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/heat/values.yaml#n448 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/heat/values.yaml#n451 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/ironic/values.yaml#n178 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/ironic/values.yaml#n181 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/keystone/values.yaml#n857 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/keystone/values.yaml#n860 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/magnum/values.yaml#n130 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/magnum/values.yaml#n133 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/manila/values.yaml#n795 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/manila/values.yaml#n798 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/mistral/values.yaml#n469 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/mistral/values.yaml#n472 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/monasca/values.yaml#n184 : worker-class: eventlet
- https://opendev.org/openstack/openstack-helm/src/branch/master/neutron/values.yaml#n2021 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/neutron/values.yaml#n2024 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/nova/values.yaml#n1516 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/nova/values.yaml#n1519 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/octavia/values.yaml#n292 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/octavia/values.yaml#n295 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/placement/values.yaml#n109 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/placement/values.yaml#n112 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/senlin/values.yaml#n177 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/senlin/values.yaml#n180 : qualname: eventlet.wsgi.server
- https://opendev.org/openstack/openstack-helm/src/branch/master/tacker/values.yaml#n585 : logger_eventletwsgi:
- https://opendev.org/openstack/openstack-helm/src/branch/master/tacker/values.yaml#n588 : qualname: eventlet.wsgi.server

***
