DELETE FROM keys WHERE end_time < now();
DELETE FROM admin_auth WHERE end_time < now();