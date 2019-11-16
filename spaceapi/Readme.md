
This tells you how to talk to the
[https://github.com/spaceapi-community/spaceapi-server-rs](spaceapi-server)

https://github.com/coredump-ch/status

https://crates.io/crates/spaceapi-server

#### Examples

Bring up the status server above and send in the following request

```
curl -v -X PUT -d value=42.1337 http://127.0.0.1:3000/sensors/raspi_temperature/

curl -v -X PUT -d value=2221 http://127.0.0.1:3000/sensors/people_now_present/

curl -v -X PUT -d value=72.22 http://127.0.0.1:3000/sensors/temperature_entrance/
```
