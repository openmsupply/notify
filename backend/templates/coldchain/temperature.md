**{{ alert_type }} temperature alert!**

**Facility**: {{ store_name }}
{% if location_name %}
**Location**: {{ location_name }}
{% endif %}
**Sensor**: {{ sensor_name }}

**Date**: {{ last_data_time | date(format="%d %b %Y") }}
**Time**: {{ last_data_time | date(format="%H:%M")}}

**Temperature**: {{ temperature }}°C
{% if reminder_number %}
**Reminder Number**: {{ reminder_number }}
{% endif %}
