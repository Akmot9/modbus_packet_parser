

#[derive(Debug)]
pub enum ModbusError {
    InvalidLength,
    InvalidFunctionCode,
    InvalidCRC,
}

#[derive(Debug)]
pub struct ModbusPacketInfo {
    pub address: u8,
    pub function_code: u8,
    pub data: Vec<u8>,
    pub crc: Option<u16>,  // CRC is None for Modbus TCP
}

pub fn parse_modbus_rtu(payload: &[u8]) -> Result<ModbusPacketInfo, ModbusError> {
    // Check for minimal length (Address, Function Code, and CRC for RTU)
    if payload.len() < 4 {
        return Err(ModbusError::InvalidLength);
    }

    // Extract the fields (assuming RTU for simplicity)
    let address = payload[0];
    let function_code = payload[1];

    // Verify the function code is valid (standard Modbus function codes range from 1 to 127)
    if function_code == 0 || function_code > 127 {
        return Err(ModbusError::InvalidFunctionCode);
    }

    // Extract data and CRC
    let data_len = payload.len() - 4; // Minus Address, Function Code, and CRC
    let data = payload[2..2 + data_len].to_vec();
    let crc = u16::from_le_bytes(payload[payload.len() - 2..].try_into().unwrap());

    // For simplicity, we'll skip CRC validation here, but in a real implementation, you'd calculate and compare it.

    Ok(ModbusPacketInfo {
        address,
        function_code,
        data,
        crc: Some(crc),
    })
}

pub fn parse_modbus_tcp(payload: &[u8]) -> Result<ModbusPacketInfo, ModbusError> {
    // Check for minimal length (Transaction ID, Protocol ID, Length, Unit ID, Function Code)
    if payload.len() < 8 {
        return Err(ModbusError::InvalidLength);
    }

    // Extract the fields
    let transaction_id = u16::from_be_bytes(payload[0..2].try_into().unwrap());
    let protocol_id = u16::from_be_bytes(payload[2..4].try_into().unwrap());
    let length = u16::from_be_bytes(payload[4..6].try_into().unwrap());
    let unit_id = payload[6];
    let function_code = payload[7];

    // Verify the function code is valid (standard Modbus function codes range from 1 to 127)
    if function_code == 0 || function_code > 127 {
        return Err(ModbusError::InvalidFunctionCode);
    }

    // Extract data
    let data = payload[8..].to_vec();

    Ok(ModbusPacketInfo {
        address: unit_id,
        function_code,
        data,
        crc: None,
    })
}

pub fn parse_modbus_rtu_over_tcp(payload: &[u8]) -> Result<ModbusPacketInfo, ModbusError> {
    if payload.len() < 8 {
        return Err(ModbusError::InvalidLength);
    }

    // Skipping the first 6 bytes for Transaction ID, Protocol ID, and Length
    parse_modbus_rtu(&payload[6..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_modbus_rtu() {
        let payload: Vec<u8> = vec![0x01, 0x03, 0x00, 0x6B, 0x00, 0x03, 0x76, 0x87];
        let result = parse_modbus_rtu(&payload).unwrap();
        assert_eq!(result.address, 0x01);
        assert_eq!(result.function_code, 0x03);
        assert_eq!(result.data, vec![0x00, 0x6B, 0x00, 0x03]);
        assert_eq!(result.crc, Some(0x8776)); // CRC is little-endian
    }

    #[test]
    fn test_parse_modbus_tcp() {
        let payload: Vec<u8> = vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x06, 0x01, 0x03, 0x00, 0x6B, 0x00, 0x03];
        let result = parse_modbus_tcp(&payload).unwrap();
        assert_eq!(result.address, 0x01);
        assert_eq!(result.function_code, 0x03);
        assert_eq!(result.data, vec![0x00, 0x6B, 0x00, 0x03]);
        assert_eq!(result.crc, None);
    }

    #[test]
    fn test_parse_modbus_rtu_over_tcp() {
        let payload: Vec<u8> = vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x01, 0x03, 0x00, 0x6B, 0x00, 0x03, 0x76, 0x87];
        let result = parse_modbus_rtu_over_tcp(&payload).unwrap();
        assert_eq!(result.address, 0x01);
        assert_eq!(result.function_code, 0x03);
        assert_eq!(result.data, vec![0x00, 0x6B, 0x00, 0x03]);
        assert_eq!(result.crc, Some(0x8776));
    }
}
