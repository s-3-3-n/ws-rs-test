pub trait Message {
    fn version() -> String;
}

// These are from my full project, which this test project is based on
// They are also released as gists under https://gist.github.com/s-3-3-n

macro_rules! api {
    ($name:ident, $ver:expr) => {
        paste::item! {
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct [<$name Message>] {
                pub ver: String,
                pub msg: $name,
            }

            impl crate::ipc::msg::Message for [<$name Message>] {
                fn version() -> String {
                    $ver.to_string()
                }
            }

            impl [<$name Message>] {
                pub fn new(msg: $name) -> Self {
                    [<$name Message>] {
                        ver: <[<$name Message>] as $crate::ipc::msg::Message>::version(),
                        msg,
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! sleep {
    (1 second) => {
        std::thread::sleep(std::time::Duration::from_secs(60));
    };
    ($num:literal sec) => {
        std::thread::sleep(std::time::Duration::from_secs($num));
    };
    ($num:literal seconds) => {
        std::thread::sleep(std::time::Duration::from_secs($num));
    };
    (1 minute) => {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    };
    ($num:literal min) => {
        std::thread::sleep(std::time::Duration::from_secs($num * 60));
    };
    ($num:literal minutes) => {
        std::thread::sleep(std::time::Duration::from_secs($num * 60));
    };
    ($num:literal ms) => {
        std::thread::sleep(std::time::Duration::from_millis($num));
    };
    ($num:literal us) => {
        std::thread::sleep(std::time::Duration::from_micros($num));
    };
    ($num:literal mu) => {
        std::thread::sleep(std::time::Duration::from_micros($num));
    };
    ($num:literal ns) => {
        std::thread::sleep(std::time::Duration::from_nanos($num));
    };
}