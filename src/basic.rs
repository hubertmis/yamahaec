use super::error::Error;
use serde::Deserialize;

pub struct Device {
    host_name: String,
}

#[derive(Debug, Deserialize)]
struct SimpleRsp {
    response_code: i32,
}

impl SimpleRsp {
    fn to_result(&self) -> Result<(), Error> {
        if self.response_code == 0 {
            Ok(())
        } else {
            Err(Error::Yamaha(self.response_code))
        }
    }
}

#[derive(Debug, Deserialize)]
struct Rsp<D> {
    response_code: i32,

    #[serde(flatten)]
    data: D,
}

impl<D> Rsp<D> {
    fn to_result(self) -> Result<Box<D>, Error> {
        if self.response_code == 0 {
            Ok(Box::new(self.data))
        } else {
            Err(Error::Yamaha(self.response_code))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    model_name: String,
    //destination: String,
    //device_id: String,
    //system_version: f32,
    //api_version: f32,
    //netmodule_version: String,
    //netmodule_checksum: String,
}

impl DeviceInfo {
    pub fn get_model_name(&self) -> &String {
        &self.model_name
    }
}

#[derive(Debug, Deserialize)]
struct InputItem {
    id: String,
//    distribution_enable: bool,
//    rename_enable: bool,
//    account_enable: bool,
//    play_info_type: String,
}

#[derive(Debug, Deserialize)]
struct SystemFeatures {
//    func_list: Vec<String>,
//    zone_num: i32,
    input_list: Vec<InputItem>,
}

#[derive(Debug, Deserialize)]
pub struct Features {
    system: SystemFeatures,
}

impl Features {
    pub fn get_input_ids(&self) -> Vec<&String> {
        let mut inputs = Vec::with_capacity(self.system.input_list.len());

        for input in &self.system.input_list {
            inputs.push(&input.id);
        }

        inputs
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "&str", into = "String")]
pub enum Power {
    On,
    Standby,
}

impl TryFrom<&str> for Power {
    type Error = Error;

    fn try_from(other: &str) -> Result<Self, Self::Error> {
        match other {
            "on" => Ok(Power::On),
            "standby" => Ok(Power::Standby),
            val => Err(Error::Value(format!("Cannot convert \"{}\" to power state", val))),
        }
    }
}

impl From<Power> for String {
    fn from(power: Power) -> Self {
        String::from(&power)
    }
}

impl From<&Power> for String {
    fn from(power: &Power) -> Self {
        match power {
            Power::On => "on".to_string(),
            Power::Standby => "standby".to_string(),
        }
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

#[derive(Debug, Deserialize)]
pub struct ZoneStatus {
//    power: Power,
//    input: String,
}

impl Device {
    fn url(&self, function: &str) -> String {
        self.url_zone("system", function)
    }

    fn url_zone(&self, zone: &str, function: &str) -> String {
        format!("http://{}/YamahaExtendedControl/v1/{}/{}", self.host_name, zone, function)
    }

    fn get_zone(zone: Option<&String>) -> String {
        if let Some(zone) = zone { 
            zone.clone()
        } else { 
            String::from("main")
        }
    }

    pub fn new(host_name: &str) -> Self {
        Self {
            host_name: String::from(host_name),
        }
    }

    pub async fn get_device_info(&self) -> Result <Box<DeviceInfo>, Error> {
        let result = reqwest::get(self.url("getDeviceInfo")).await?
            .json::<Rsp<DeviceInfo>>().await?;

        result.to_result()
    }

    pub async fn get_features(&self) -> Result <Box<Features>, Error> {
        let result = reqwest::get(self.url("getFeatures")).await?
            .json::<Rsp<Features>>().await?;

        result.to_result()
    }

    pub async fn get_status(&self, zone: Option<&String>) -> Result <Box<ZoneStatus>, Error> {
        let result = reqwest::get(self.url_zone(&Self::get_zone(zone), "getStatus")).await?
            .json::<Rsp<ZoneStatus>>().await?;

        result.to_result()
    }

    pub async fn set_power(&self, zone: Option<&String>, power: Power)  -> Result <(), Error> {
        let zone = Self::get_zone(zone);
        let url = format!("{}?power={}", self.url_zone(&zone, "setPower"), power);

        let result = reqwest::get(&url).await?
            .json::<SimpleRsp>().await?;

        result.to_result()
    }

    pub async fn set_input(&self, zone: Option<&String>, input: &str, mode: Option<&String>)  -> Result <(), Error> {
        let zone = Self::get_zone(zone);
        let mode = if let Some(mode) = mode { format!("&mode={}", mode) } else { String::new() };
        let url = format!("{}?input={}{}", self.url_zone(&zone, "setInput"), input, mode);

        let result = reqwest::get(&url).await?
            .json::<SimpleRsp>().await?;

        result.to_result()
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[tokio::test]
    async fn test_get_device_info() {
        let d = Device::new("sypialnia.local");
        let info = d.get_device_info().await;
        println!("Device info: {:?}", info);
    }

    #[tokio::test]
    async fn test_get_features() {
        let d = Device::new("sypialnia.local");
        let r = d.get_features().await;
        println!("Features: {:?}", r);
    }

    #[tokio::test]
    async fn test_get_status() {
        let d = Device::new("sypialnia.local");
        let r = d.get_status(None).await;
        println!("Status: {:?}", r);
    }

    #[tokio::test]
    async fn test_set_power() {
        let d = Device::new("sypialnia.local");
        let r = d.set_power(None, Power::Standby).await;
        println!("Status: {:?}", r);
    }

    #[tokio::test]
    async fn test_set_input() {
        let d = Device::new("sypialnia.local");
        let r = d.set_input(None, "optical", None).await;
        println!("Status: {:?}", r);
    }
    */
}
