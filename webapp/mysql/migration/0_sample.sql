-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
CREATE INDEX idx_users_username ON users (username);
CREATE INDEX idx_users_password ON users (password);
CREATE INDEX idx_tow_trucks_id_driver_id ON tow_trucks (id, driver_id);
CREATE INDEX idx_locations_tow_truck_id_timestamp ON locations (tow_truck_id, timestamp);
CREATE INDEX idx_tow_trucks_status_id ON tow_trucks (status, id);
CREATE INDEX idx_tow_trucks_status_area_id_id ON tow_trucks (status, area_id, id);
CREATE INDEX idx_orders_status_order_time ON orders (status, order_time);
CREATE INDEX idx_nodes_area_id ON nodes (area_id);
CREATE INDEX idx_tow_trucks_id ON tow_trucks (id);
CREATE INDEX idx_sessions_session_token ON sessions (session_token);
CREATE INDEX idx_orders_node_id ON orders (node_id);
CREATE INDEX idx_orders_client_id ON orders (client_id);
CREATE INDEX idx_orders_dispatcher_id ON orders (dispatcher_id);
CREATE INDEX idx_orders_tow_truck_id ON orders (tow_truck_id);
CREATE INDEX idx_dispatchers_id ON dispatchers (id);
CREATE INDEX idx_dispatchers_user_id ON dispatchers (user_id);