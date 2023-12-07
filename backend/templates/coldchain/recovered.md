**{% if old_status == "NoData" %}✅ Monitoring connection restored{% else %}✅ Sensor is now ok!{% endif %}**

**Facility**: {{ store_name }}
{% if location_name %}
**Location**: {{ location_name }}
{% endif %}
**Sensor**: {{ sensor_name }}

**Date**: {{ last_data_time | date(format="%d %b %Y") }}
**Time**: {{ last_data_time | date(format="%H:%M")}}

**Temperature**: {{ temperature }} °C
**Last data received**: {{ data_age }} ago
