Frequently Asked Questions


## What happens to missed notifications?
If the server isn't running at the scheduled time, the last notification will be sent when the server starts up again. The notification will be calculated at the time it is sent, so if for example, you have a report that runs on the first day of each month but the server isn't running on the first day of the month, the report will be calculated with the data available at the send time.
It might be possible to write a query to look at the historial data, but this is not inherent in the system.

Any notification iterations in between will be missed. E.g. if you have a daily notification setup, but the server is offline for 2 days, when you start the server up, only one notification will be generated.

## How do I set up a notification for the end of the month?
You can set up a notification to run on the last day of the month by selecting the startdate in a month with 31 days and monthly schedule. The notification will run on the 31st of each month, and if the month has less than 31 days, it will run on the last day of the month.
Or you can run at midnight on the first day of the new month (which might be more what you intend to do?)
