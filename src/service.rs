use crate::error::ParseError;

#[derive(Debug, PartialEq)]
pub struct Service {
    pub clear: bool,
    pub protocol_id: Option<String>,
    pub alt_authority: Option<AltAuthority>,
    pub max_age: Option<i32>,
    pub persist: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub struct AltAuthority {
    pub host: Option<String>,
    pub port: Option<String>,
}

pub fn parse(s: &str) -> Result<Vec<Service>, ParseError> {
    let mut ret = Vec::new();
    if s == "clear" {
        ret.push(Service {
            clear: true,
            protocol_id: None,
            alt_authority: None,
            max_age: None,
            persist: None,
        });
        return Ok(ret);
    }

    let services = s.split(",");
    for svc_string in services {
        let mut svc = Service {
            clear: false,
            protocol_id: None,
            alt_authority: None,
            max_age: None,
            persist: None,
        };
        if svc_string.len() == 0 {
            return Err(ParseError::InvalidParameter(svc_string.to_string()));
        }
        let params = svc_string.split(";");
        for kv in params {
            let raw_kv = kv.trim();
            if raw_kv.len() == 0 {
                continue;
            }
            let kv_split: Vec<&str> = dbg!(raw_kv).split("=").collect();
            if kv_split.len() != 2 {
                return Err(ParseError::InvalidParameter(raw_kv.to_string()));
            }
            let k = kv_split[0];
            let v = kv_split[1];
            match k {
                "ma" => {
                    let ma = v
                        .parse::<i32>()
                        .map_err(|_| ParseError::InvalidMaValue(v.to_string()))?;
                    svc.max_age = Some(ma);
                }
                "persist" => {
                    let persist = v
                        .parse::<i32>()
                        .map_err(|_| ParseError::InvalidPersistValue(v.to_string()))?;
                    if persist != 1 {
                        continue;
                    }
                    svc.persist = Some(1);
                }
                _ => {
                    let raw_value = v.trim_matches('"');
                    let alt_auth_split: Vec<&str> = raw_value.split(":").collect();
                    if alt_auth_split.len() != 2 {
                        return Err(ParseError::InvalidAltAuthorityValue(raw_value.to_string()));
                    }
                    svc.protocol_id = Some(k.to_string());
                    let host = alt_auth_split[0].to_string();
                    let host = match host.len() {
                        0 => None,
                        _ => Some(host),
                    };
                    let port = alt_auth_split[1].to_string();
                    let port = match port.len() {
                        0 => None,
                        _ => Some(port),
                    };
                    svc.alt_authority = match (host.is_none(), port.is_none()) {
                        (true, true) => None,
                        _ => Some(AltAuthority { host, port }),
                    };
                }
            }
        }
        ret.push(svc);
    }
    Ok(ret)
}
