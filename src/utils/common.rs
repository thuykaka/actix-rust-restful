use actix_web::dev::ServiceRequest;

pub fn get_client_ip(req: &ServiceRequest) -> String {
    // 1. X-Forwarded-For
    if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_for) = forwarded_for.to_str() {
            // get first ip
            if let Some(ip) = forwarded_for.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }

    // 2. X-Real-IP
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(real_ip) = real_ip.to_str() {
            return real_ip.trim().to_string();
        }
    }

    // 3. peer_addr (no proxy)
    if let Some(peer_ip) = req.connection_info().peer_addr() {
        return peer_ip.to_string();
    }

    // fallback
    "unknown".to_string()
}
