SELECT uuid FROM admin_auth WHERE auth_key = $1 AND end_time > now();