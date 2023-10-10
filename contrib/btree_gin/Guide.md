# What is this extension?
>*B-tree-similar GIN operator class.*

`btree_gin` is an extension that provides Generalized Inverted Index (GIN) operator classes to simulate B-tree-like behavior for a wide variety of data types, such as int2, int4, int8, float4, float8, timestamp, date, oid, text, bytea, macaddr, inet, uuid, bool, bpchar, and all enum types, among others.

# When should you use it?

This extension is tailored for specialized use cases, not necessarily general indexing. Some reasons you might consider using btree_gin include:
- GIN Testing
- Developing Other GIN Operator Classes
- Multicolumn GIN Indexing

# Example use case.

Imagine a large train station that offers free WiFi to passengers. The station's database logs various information about devices that connect to the WiFi, such as the device's MAC address (**`macaddr`** type), the timestamp of connection (**`timestamp with time zone`**), and possibly textual descriptions of the device, such as the device name (**`text`** type) or the type of device (smartphone, laptop, etc.). 

The train station often runs queries that filter based on both the time of connection (to analyze peak usage times) and the type of device (to understand the most common devices being used). For instance, they might want to know how many smartphones connected to the WiFi between 8 AM and 9 AM.

Using the **btree_gin** extension, the station can create a multicolumn GIN index that encompasses both the timestamp of connection and the device type. This setup can potentially offer performance benefits over maintaining separate B-tree and GIN indexes, especially when running combined queries on both columns.

# Example test script.

-- Create example table, wifi_log, and populate with test data
CREATE TABLE wifi_log (
    id SERIAL PRIMARY KEY,
    mac_address MACADDR NOT NULL,
    connection_time TIMESTAMPTZ NOT NULL,
    device_name TEXT,
    device_type TEXT
);

INSERT INTO wifi_log (mac_address, connection_time, device_name, device_type) VALUES
('08:00:27:4B:C7:FA', '2023-10-06 08:15:00+00', 'John’s iPhone', 'smartphone'),
('BC:1A:EA:2F:12:34', '2023-10-06 08:35:00+00', 'Jane’s Laptop', 'laptop'),
('DE:AD:BE:EF:00:01', '2023-10-06 09:05:00+00', 'Mike’s Smartphone', 'smartphone'),
('00:1A:2B:3C:4D:5E', '2023-10-06 07:45:00+00', 'Linda’s Tablet', 'tablet'),
('A1:B2:C3:D4:E5:F6', '2023-10-06 08:10:00+00', 'Dave’s Smartwatch', 'smartwatch');

-- Create GIN index

CREATE INDEX idx_wifi_log_gin ON wifi_log USING GIN(connection_time, device_type);

-- Run query to count the number of smartphones that connected between 8 AM and 9 AM
SELECT 
    COUNT(*) AS smartphone_count 
FROM wifi_log 
WHERE connection_time BETWEEN '2023-10-06 08:00:00+00' AND '2023-10-06 09:00:00+00'
AND device_type = 'smartphone';

-- Drop table(s) and affiliate items to complete example
DROP TABLE wifi_log CASCADE;
