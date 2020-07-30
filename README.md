# acc_director_lite

## Broadcast client for Assetto Corsa Competizione

This is just a barely finished project i did to learn Rust, so don't blame me if you see bad code :D

The application need to be launched AFTER you joined a single player or multiplayer game session.

Configuration settings are in the /accd_core/config/default.cfg file, edit it with your name, password and destination address.
```
protocol_version = 4
display_name = Your name
connection_password = asd
update_interval = 250
command_password =
bind_address = 0.0.0.0:3400
destination_address = 127.0.0.1:9000
```
destination_address is composed by an ip address and a port: 

ip address ```127.0.0.1```
port ```9000```

destination address port, connection password and command password need to be the same as in your

"\Documents\Assetto Corsa Competizione\Config\broadcasting.json" file.

For example:
```
{
    "updListenerPort": 9000,   
    "connectionPassword": "asd",
    "commandPassword": ""
}
```
