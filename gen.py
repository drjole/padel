import datetime
import json

start_date = datetime.datetime(2024, 8, 23)
end_date = datetime.datetime(2024, 12, 27)

date = start_date
data = []

while True:
    if date > end_date:
        break
    data.append({
        "start_time": date + datetime.timedelta(hours=15),
        "end_time": date + datetime.timedelta(hours=16),
    })
    data.append({
        "start_time": date + datetime.timedelta(hours=16),
        "end_time": date + datetime.timedelta(hours=17),
    })
    date += datetime.timedelta(days=7)

with open("data/signups.json", "w") as f:
    f.write(json.dumps(data, indent=2, default=lambda d: d.isoformat()))
