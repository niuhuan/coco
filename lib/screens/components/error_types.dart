const ERROR_TYPE_NETWORK = "NETWORK_ERROR";
const ERROR_TYPE_PERMISSION = "PERMISSION_ERROR";
const ERROR_TYPE_TIME = "TIME_ERROR";

// 错误的类型, 方便照展示和谐的提示
String errorType(String error) {
  // EXCEPTION
  // Get "https://****": net/http: TLS handshake timeout
  // Get "https://****": proxyconnect tcp: dial tcp 192.168.123.217:1080: connect: connection refused
  // Get "https://****": context deadline exceeded (Client.Timeout exceeded while awaiting headers)
  if (error.contains("timeout") ||
      error.contains("connection refused") ||
      error.contains("deadline") ||
      error.contains("connection abort")) {
    return ERROR_TYPE_NETWORK;
  }
  if (error.contains("permission denied")) {
    return ERROR_TYPE_PERMISSION;
  }
  if (error.contains("time is not synchronize")) {
    return ERROR_TYPE_TIME;
  }
  return "";
}
