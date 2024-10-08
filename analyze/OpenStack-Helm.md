# Analysis for Team: OpenStack-Helm Project: openstack-helm
The code snippet provided appears to be a list of various OpenStack projects and their respective values files, which are used to configure the project's deployment. The specific lines highlighted in green (`logger_eventletwsgi`) point to settings related to logging for each project.

To summarize:

*   `eventlet.wsgi.server` is set as the logger name for several projects, including Cinder, Designate, Glance, Heat, Ironic, Keystone, Magnum, Manila, Mistral, Monasca, Neutron, Nova, Octavia, Placement, Senlin, and Tacker.
*   The performance impact of using `eventlet.wsgi.server` is mentioned in the Nginx-sidecar documentation.
*   These settings are likely used to configure the logging behavior for each project when using the Eventlet WSGI server.

To answer the question: "What does this code snippet do?", it appears that this code snippet is a list of configuration values for various OpenStack projects, specifically related to logging and performance.

Occurrences Found:
https://opendev.org/openstack/openstack-helm/src/branch/master/aodh/values.yaml#n506 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/aodh/values.yaml#n509 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/barbican/values.yaml#n462 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/barbican/values.yaml#n465 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/cinder/values.yaml#n884 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/cinder/values.yaml#n887 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/designate/values.yaml#n501 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/designate/values.yaml#n504 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/doc/source/specs/nginx-sidecar.rst#n16 : performance impact, for example, eventlet.
https://opendev.org/openstack/openstack-helm/src/branch/master/glance/values.yaml#n344 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/glance/values.yaml#n347 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/heat/values.yaml#n448 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/heat/values.yaml#n451 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/ironic/values.yaml#n178 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/ironic/values.yaml#n181 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/keystone/values.yaml#n857 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/keystone/values.yaml#n860 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/magnum/values.yaml#n130 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/magnum/values.yaml#n133 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/manila/values.yaml#n795 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/manila/values.yaml#n798 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/mistral/values.yaml#n469 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/mistral/values.yaml#n472 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/monasca/values.yaml#n184 : worker-class: eventlet
https://opendev.org/openstack/openstack-helm/src/branch/master/neutron/values.yaml#n2021 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/neutron/values.yaml#n2024 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/nova/values.yaml#n1516 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/nova/values.yaml#n1519 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/octavia/values.yaml#n292 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/octavia/values.yaml#n295 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/placement/values.yaml#n109 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/placement/values.yaml#n112 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/senlin/values.yaml#n177 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/senlin/values.yaml#n180 : qualname: eventlet.wsgi.server
https://opendev.org/openstack/openstack-helm/src/branch/master/tacker/values.yaml#n585 : logger_eventletwsgi:
https://opendev.org/openstack/openstack-helm/src/branch/master/tacker/values.yaml#n588 : qualname: eventlet.wsgi.server
