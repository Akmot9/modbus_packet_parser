from pymodbus.client import ModbusTcpClient
import logging

# Configure logging
logging.basicConfig(level=logging.DEBUG)

# Define Modbus server details
modbus_server_ip = '0.0.0.0'  # Replace with your Modbus server IP address
modbus_server_port = 502

# Create a Modbus client
client = ModbusTcpClient(modbus_server_ip, port=modbus_server_port)

# Connect to the Modbus server
try:
    connection = client.connect()
    if connection:
        logging.info("Connected to Modbus server")

        # Read Holding Registers (Function code 0x03)
        address = 0    # The address of the first register
        count = 10     # Number of registers to read
        response = client.read_holding_registers(address, count)

        if response.isError():
            logging.error("Error reading registers: %s", response)
        else:
            logging.info("Register values: %s", response.registers)

        # Close the connection
        client.close()
    else:
        logging.error("Unable to connect to Modbus server")
except Exception as e:
    logging.error("Exception occurred: %s", e)
