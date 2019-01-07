# Design

Anatomy of a long formatted log entry:

* Generalized terms are enclosed in square [] brackets.
* Specific key names are presented with no modification. 

```
                                                                                                                                           ┌src.func─┐
                                                                                                                                           └──┐   ┌──┘
                        ┌log Level─┐    ┌component─┐                                                                              ┌src.line─┐ │   │
                        └──┐    ┌──┘    └┐        ┌┘                                                                              └──┐   ┌──┘ │   │
 ┌──UTC or local time───┐  │    │ ┌app─┐ │        │ ┌port┐                                          ┌───────────src.file───────────┐ │   │    │   │
 │                      │  │    │ │name│ │        │ │    │    ┌─────────────hostname─────────────┐ ┌┼────────────────────src───────┼─┼───┼────┼───┼┐  ┌───msg────┐ ┌─────────────────────────────────────────────────[extra params]──────────────────────────────────────────────────────────────┐
[2018-12-04T00:00:03.114Z]  INFO: muskie/HttpServer/844164 on efeacff8-a36d-4ac7-8f80-fc8e21bcf944 (/opt/smartdc/muskie/lib/audit.js:32338 in audit): handled: 302 (audit=true, _audit=true, operation=get100, remotePort=57919, reqHeaderLength=49, resHeaderLength=119, latency=1, route=get100)
    GET / HTTP/1.1                  ┐
    host: manta.us-east.scloud.host ├─── HTTP request 
    connection: keep-alive          ┘
    --
    HTTP/1.1 302 Found                         ┐
    content-length: 0                          │
    connection: keep-alive                     ├─── HTTP response
    date: Tue, 04 Dec 2018 00:00:03 GMT        │
    location: http://apidocs.joyent.com/manta/ │
    server: Manta                              ┘
    --
    req.caller: {
      "login": null,
      "uuid": null,
      "groups": null,
      "user": null
    }
    --
    req.timers: {
      "redirect": 531
    }

``` 