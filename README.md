<div align="center">

# Flow

---

### A Minecraft Reverse Proxy

</div>

An ultra-lightweight, high-performance Minecraft reverse proxy built from the ground up in Rust. It functions as an intelligent edge-gate that intercepts incoming player connections, decrypts and processes initial protocol states, and establishes a secure tunnel to your backend server with near-zero overhead.

The project runs on a pure asynchronous state machine, consuming less than 1 MB of RAM (~850 KB) in idle. The compiled executable size is only ~230 KB, making it significantly lighter than traditional Java alternatives.

## Features
- Reverse Proxy Core - Asynchronous TCP stream proxying between clients and the backend server.
- In-flight Decryption - Decrypts and processes the encrypted protocol packets natively before entering the Play phase.
- Ultra-low Footprint - Extremely light on system resources (~230 KB binary size, <1 MB RAM), ideal for high-density edge deployments.

## Configuration (env)
- LISTENER_ADDRESS; 
	The host and port the proxy will listen on for incoming player connections
  Example: 0.0.0.0:25565
- CONNECTION_ADDRESS;
  The address of the backend Minecraft server where traffic should be routed
  Example: 127.0.0.1:25565
