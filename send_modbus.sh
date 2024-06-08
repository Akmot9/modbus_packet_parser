#!/bin/bash

# Modbus TCP packet to read holding registers (function code 0x03)
# Transaction Identifier: 0x0001
# Protocol Identifier: 0x0000
# Length: 0x0006
# Unit Identifier: 0x01
# Function Code: 0x03
# Starting Address: 0x006B (107)
# Quantity of Registers: 0x0003 (3)

# Packet structure:
# | Transaction ID (2 bytes) | Protocol ID (2 bytes) | Length (2 bytes) | Unit ID (1 byte) | Function Code (1 byte) | Starting Address (2 bytes) | Quantity of Registers (2 bytes) |
modbus_packet=$(echo -n -e '\x00\x01\x00\x00\x00\x06\x01\x03\x00\x6B\x00\x03')

# IP address of the Modbus server
modbus_server_ip="0.0.0.0"
modbus_server_port=502

# Sending the Modbus TCP packet using netcat
echo -n "$modbus_packet" | nc $modbus_server_ip $modbus_server_port

# Output the sent packet in a readable format
echo "Sent Modbus packet:"
echo "$modbus_packet" | xxd
