# Divergences from node-bunyan

#### Indentation on empty lines is trimmed

Bunyan line:
```json
{"name":"myservice","hostname":"example.com","pid":123,"level":30,"client_res":{"statusCode":200,"headers":{"content-type":"text/plain","content-length":0,"date":"Sat, 07 Mar 2015 06:58:43 GMT"},"body":"hello"},"msg":"hello","time":"2016-02-10T07:28:41.419Z","v":0}
```

Spaces have been replaced with the symbol: ␣

node-bunyan output:
```
[2016-02-10T07:28:41.419Z]␣␣INFO:␣myservice/123␣on␣example.com:␣hello
␣␣␣␣HTTP/1.1␣200␣OK
␣␣␣␣content-type:␣text/plain
␣␣␣␣content-length:␣0
␣␣␣␣date:␣Sat,␣07␣Mar␣2015␣06:58:43␣GMT
␣␣␣␣
␣␣␣␣hello
```

bunyan-view␣(rust)␣output:
```
[2016-02-10T07:28:41.419Z]␣␣INFO:␣myservice/123␣on␣example.com:␣hello
␣␣␣␣HTTP/1.1␣200␣OK
␣␣␣␣content-type:␣text/plain
␣␣␣␣content-length:␣0
␣␣␣␣date:␣Sat,␣07␣Mar␣2015␣06:58:43␣GMT

␣␣␣␣hello
```

#### Source references without line numbers are displayed without a reference to the line number

Bunyan line:
```json
{"name":"mls","hostname":"MBP","pid":60876,"component":"MantaClient","path":"/foo/stor","req_id":"021138ff-e2f9-4085-af18-f543adef05d2","level":20,"err":{"message":"foo does not exist","name":"AccountDoesNotExistError","stack":"AccountDoesNotExistError: foo does not exist\n    at ClientRequest.onResponse (/usr/local/lib/node_modules/manta/node_modules/restify-clients/lib/HttpClient.js:217:26)\n    at ClientRequest.g (events.js:273:16)\n    at emitOne (events.js:90:13)\n    at ClientRequest.emit (events.js:182:7)\n    at HTTPParser.parserOnIncomingClient (_http_client.js:458:21)\n    at HTTPParser.parserOnHeadersComplete (_http_common.js:103:23)\n    at TLSSocket.socketOnData (_http_client.js:348:20)\n    at emitOne (events.js:90:13)\n    at TLSSocket.emit (events.js:182:7)\n    at readableAddChunk (_stream_readable.js:153:18)","code":"AccountDoesNotExist"},"msg":"get: error","time":"2018-11-27T11:53:15.458Z","src":{"file":"/usr/local/lib/node_modules/manta/lib/client.js"},"v":0}
```

node-bunyan output:
```
[2018-11-27T11:53:15.458Z] DEBUG: mls/MantaClient/60876 on MBP (/usr/local/lib/node_modules/manta/lib/client.js:NaN): get: error (req_id=021138ff-e2f9-4085-af18-f543adef05d2, path=/foo/stor, err.code=AccountDoesNotExist)
    AccountDoesNotExistError: foo does not exist
        at ClientRequest.onResponse (/usr/local/lib/node_modules/manta/node_modules/restify-clients/lib/HttpClient.js:217:26)
        at ClientRequest.g (events.js:273:16)
        at emitOne (events.js:90:13)
        at ClientRequest.emit (events.js:182:7)
        at HTTPParser.parserOnIncomingClient (_http_client.js:458:21)
        at HTTPParser.parserOnHeadersComplete (_http_common.js:103:23)
        at TLSSocket.socketOnData (_http_client.js:348:20)
        at emitOne (events.js:90:13)
        at TLSSocket.emit (events.js:182:7)
        at readableAddChunk (_stream_readable.js:153:18)
```

bunyan-view (rust) output:
```
[2018-11-27T11:53:15.458Z] DEBUG: mls/MantaClient/60876 on MBP (/usr/local/lib/node_modules/manta/lib/client.js): get: error (req_id=021138ff-e2f9-4085-af18-f543adef05d2, path=/foo/stor, err.code=AccountDoesNotExist)
    AccountDoesNotExistError: foo does not exist
        at ClientRequest.onResponse (/usr/local/lib/node_modules/manta/node_modules/restify-clients/lib/HttpClient.js:217:26)
        at ClientRequest.g (events.js:273:16)
        at emitOne (events.js:90:13)
        at ClientRequest.emit (events.js:182:7)
        at HTTPParser.parserOnIncomingClient (_http_client.js:458:21)
        at HTTPParser.parserOnHeadersComplete (_http_common.js:103:23)
        at TLSSocket.socketOnData (_http_client.js:348:20)
        at emitOne (events.js:90:13)
        at TLSSocket.emit (events.js:182:7)
        at readableAddChunk (_stream_readable.js:153:18)
```

#### Source references that do not contain a JSON object and only contain a string are displayed

Bunyan line:
```json
{"name":"mls","hostname":"MBP","pid":60876,"component":"MantaClient","path":"/foo/stor","req_id":"021138ff-e2f9-4085-af18-f543adef05d2","level":20,"err":{"message":"foo does not exist","name":"AccountDoesNotExistError","stack":"AccountDoesNotExistError: foo does not exist\n    at ClientRequest.onResponse (/usr/local/lib/node_modules/manta/node_modules/restify-clients/lib/HttpClient.js:217:26)\n    at ClientRequest.g (events.js:273:16)\n    at emitOne (events.js:90:13)\n    at ClientRequest.emit (events.js:182:7)\n    at HTTPParser.parserOnIncomingClient (_http_client.js:458:21)\n    at HTTPParser.parserOnHeadersComplete (_http_common.js:103:23)\n    at TLSSocket.socketOnData (_http_client.js:348:20)\n    at emitOne (events.js:90:13)\n    at TLSSocket.emit (events.js:182:7)\n    at readableAddChunk (_stream_readable.js:153:18)","code":"AccountDoesNotExist"},"msg":"get: error","time":"2018-11-27T11:53:15.458Z","src":"/usr/local/lib/node_modules/manta/lib/client.js:806","v":0}
```

node-bunyan output:
```
[2018-11-27T11:53:15.458Z] DEBUG: mls/MantaClient/60876 on MBP: get: error (req_id=021138ff-e2f9-4085-af18-f543adef05d2, path=/foo/stor, err.code=AccountDoesNotExist)
    AccountDoesNotExistError: foo does not exist
        at ClientRequest.onResponse (/usr/local/lib/node_modules/manta/node_modules/restify-clients/lib/HttpClient.js:217:26)
        at ClientRequest.g (events.js:273:16)
        at emitOne (events.js:90:13)
        at ClientRequest.emit (events.js:182:7)
        at HTTPParser.parserOnIncomingClient (_http_client.js:458:21)
        at HTTPParser.parserOnHeadersComplete (_http_common.js:103:23)
        at TLSSocket.socketOnData (_http_client.js:348:20)
        at emitOne (events.js:90:13)
        at TLSSocket.emit (events.js:182:7)
        at readableAddChunk (_stream_readable.js:153:18)
```

bunyan-view (rust) output:
```
[2018-11-27T11:53:15.458Z] DEBUG: mls/MantaClient/60876 on MBP (/usr/local/lib/node_modules/manta/lib/client.js:806): get: error (req_id=021138ff-e2f9-4085-af18-f543adef05d2, path=/foo/stor, err.code=AccountDoesNotExist)
    AccountDoesNotExistError: foo does not exist
        at ClientRequest.onResponse (/usr/local/lib/node_modules/manta/node_modules/restify-clients/lib/HttpClient.js:217:26)
        at ClientRequest.g (events.js:273:16)
        at emitOne (events.js:90:13)
        at ClientRequest.emit (events.js:182:7)
        at HTTPParser.parserOnIncomingClient (_http_client.js:458:21)
        at HTTPParser.parserOnHeadersComplete (_http_common.js:103:23)
        at TLSSocket.socketOnData (_http_client.js:348:20)
        at emitOne (events.js:90:13)
        at TLSSocket.emit (events.js:182:7)
        at readableAddChunk (_stream_readable.js:153:18)
```


#### Request trailer headers are displayed along with the request
Bunyan line:
```json
{"name":"amon-master","hostname":"9724a190-27b6-4fd8-830b-a574f839c67d","pid":12859,"audit":true,"level":30,"remoteAddress":"10.2.207.2","remotePort":50394,"req_id":"cce79d15-ffc2-487c-a4e4-e940bdaac31e","req":{"method":"HEAD","url":"/agentprobes?agent=ccf92af9-0b24-46b6-ab60-65095fdd3037","headers":{"accept":"application/json","content-type":"application/json","host":"10.2.207.16","connection":"keep-alive"},"httpVersion":"1.1","trailers":{"expires":"Wed, 21 Oct 2015 07:28:00 GMT","x-custom":"my value"},"version":"*"},"res":{"statusCode":200,"headers":{"content-md5":"11FxOYiYfpMxmANj4kGJzg==","access-control-allow-origin":"*","access-control-allow-headers":"Accept, Accept-Version, Content-Length, Content-MD5, Content-Type, Date, X-Api-Version","access-control-allow-methods":"HEAD","access-control-expose-headers":"X-Api-Version, X-Request-Id, X-Response-Time","connection":"Keep-Alive","date":"Wed, 08 Aug 2012 10:25:47 GMT","server":"Amon Master/1.0.0","x-request-id":"cce79d15-ffc2-487c-a4e4-e940bdaac31e","x-response-time":3},"trailer":false},"route":{"name":"HeadAgentProbes","version":false},"latency":3,"secure":false,"_audit":true,"msg":"HeadAgentProbes handled: 200","time":"2012-08-08T10:25:47.637Z","v":0}
```

node-bunyan output:
```
[2012-08-08T10:25:47.637Z]  INFO: amon-master/12859 on 9724a190-27b6-4fd8-830b-a574f839c67d: HeadAgentProbes handled: 200 (req_id=cce79d15-ffc2-487c-a4e4-e940bdaac31e, audit=true, remoteAddress=10.2.207.2, remotePort=50394, latency=3, secure=false, _audit=true, req.version=*)
    HEAD /agentprobes?agent=ccf92af9-0b24-46b6-ab60-65095fdd3037 HTTP/1.1
    accept: application/json
    content-type: application/json
    host: 10.2.207.16
    connection: keep-alive
    --
    HTTP/1.1 200 OK
    content-md5: 11FxOYiYfpMxmANj4kGJzg==
    access-control-allow-origin: *
    access-control-allow-headers: Accept, Accept-Version, Content-Length, Content-MD5, Content-Type, Date, X-Api-Version
    access-control-allow-methods: HEAD
    access-control-expose-headers: X-Api-Version, X-Request-Id, X-Response-Time
    connection: Keep-Alive
    date: Wed, 08 Aug 2012 10:25:47 GMT
    server: Amon Master/1.0.0
    x-request-id: cce79d15-ffc2-487c-a4e4-e940bdaac31e
    x-response-time: 3
    --
    route: {
      "name": "HeadAgentProbes",
      "version": false
    }
```

bunyan-view (rust) output:
```
[2012-08-08T10:25:47.637Z]  INFO: amon-master/12859 on 9724a190-27b6-4fd8-830b-a574f839c67d: HeadAgentProbes handled: 200 (req_id=cce79d15-ffc2-487c-a4e4-e940bdaac31e, audit=true, remoteAddress=10.2.207.2, remotePort=50394, latency=3, secure=false, _audit=true, req.version=*)
    HEAD /agentprobes?agent=ccf92af9-0b24-46b6-ab60-65095fdd3037 HTTP/1.1
    accept: application/json
    content-type: application/json
    host: 10.2.207.16
    connection: keep-alive
    expires: Wed, 21 Oct 2015 07:28:00 GMT
    x-custom: my value
    --
    HTTP/1.1 200 OK
    content-md5: 11FxOYiYfpMxmANj4kGJzg==
    access-control-allow-origin: *
    access-control-allow-headers: Accept, Accept-Version, Content-Length, Content-MD5, Content-Type, Date, X-Api-Version
    access-control-allow-methods: HEAD
    access-control-expose-headers: X-Api-Version, X-Request-Id, X-Response-Time
    connection: Keep-Alive
    date: Wed, 08 Aug 2012 10:25:47 GMT
    server: Amon Master/1.0.0
    x-request-id: cce79d15-ffc2-487c-a4e4-e940bdaac31e
    x-response-time: 3
    --
    route: {
      "name": "HeadAgentProbes",
      "version": false
    }
```

#### Error message stack with empty arrays is shown as empty arrays and not `[object Object]`
Bunyan line:
```json
{"name":"AKP48","module":"Server","hostname":"AKP48.akpwebdesign.com","pid":32421,"level":60,"err":{"message":"Function.prototype.apply: Arguments list has wrong type","name":"TypeError","stack":[{},{},{},{},{},{}]},"msg":"Exception caught: TypeError: Function.prototype.apply: Arguments list has wrong type","time":"2015-04-13T04:03:46.206Z","v":0}
```

node-bunyan output:
```
[2015-04-13T04:03:46.206Z] FATAL: AKP48/32421 on AKP48.akpwebdesign.com: Exception caught: TypeError: Function.prototype.apply: Arguments list has wrong type (module=Server)
    [object Object],[object Object],[object Object],[object Object],[object Object],[object Object]
```

bunyan-view (rust) output:
```
[2015-04-13T04:03:46.206Z] FATAL: AKP48/32421 on AKP48.akpwebdesign.com: Exception caught: TypeError: Function.prototype.apply: Arguments list has wrong type (module=Server)
    {}
    {}
    {}
    {}
    {}
    {}
```