**📶 Monitoring connection lost!**

**Facility**: {{ store_name }}
{% if location_name %}
**Location**: {{ location_name }}
{% endif %}
**Sensor**: {{ sensor_name }}

**Date**: {{ last_data_time | date(format="%d %b %Y") }}
**Time**: {{ last_data_time | date(format="%H:%M")}}

**Last data received**: {{ data_age }} ago
{% if reminder_number %}
**Reminder number**: {{ reminder_number }}
{% endif %}
