<div align="center">

# Flow

---

### A Minecraft Reverse Proxy

</div>

An ultra-lightweight, high-performance Minecraft reverse proxy built from the ground up in Rust. It functions as an intelligent edge-gate that intercepts incoming player connections, processes initial protocol states, and establishes a secure tunnel to your backend server with near-zero overhead.

The project runs on a pure asynchronous state machine, consuming less than 1 MB of RAM (~850 KB) in idle. The compiled executable size is only ~230 KB, making it significantly lighter than traditional Java or Goland alternatives.

## Features
- Reverse Proxy Core - Asynchronous TCP stream proxying between clients and the backend server.
- Ultra-low Footprint - Extremely light on system resources (~250 KB binary size, <1 MB RAM), ideal for high-density edge deployments.
- Multi-routing - Gain the ability to route traffic based on the domain and port through which the user has connected.

## Configuration (env)
- LISTENER; 
	The host and port the proxy will listen on for incoming player connections
	Example: set LISTENER=0.0.0.0:25565
- ROAMING;
	Routing string. Format: "listen:port;forward:port;listen2:port;forward2:port".
	The semicolon (;) character acts as a separator between the listening and forwarding addresses.
	Example: set ROAMING=best.server.com:25565;cool.server.com:25566

## Similar Projects
- https://github.com/itzg/mc-router
- https://github.com/haveachin/infrared
