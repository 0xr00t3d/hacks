use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone)]
pub struct Injector {
    pub request: Url,
    pub keep_value: bool,
}

pub trait Urlinjector {
    fn url_value(&self, payload: &str) -> HashMap<String, Vec<Url>>;
    fn set_urlvalue(&self, param: &str, payload: &str) -> Url;
}

impl Urlinjector for Injector {
    fn set_urlvalue(&self, param: &str, payload: &str) -> Url {
        let mut url = self.request.clone();
        let mut final_params = HashMap::new();

        url.query_pairs()
            .into_iter()
            .collect::<HashMap<_, _>>()
            .iter()
            .for_each(|(k, v)| {
                if k == param {
                    final_params.insert(k.to_string(), {
                        if self.keep_value == true {
                            format!("{}{}", v.to_string(), payload)
                        } else {
                            format!("{}", payload)
                        }
                    });
                } else {
                    final_params.insert(k.to_string(), v.to_string());
                }
            });
        url.query_pairs_mut().clear();
        url.query_pairs_mut().extend_pairs(final_params);
        url
    }

    fn url_value(&self, payload: &str) -> HashMap<String, Vec<Url>> {
        let url = self.request.clone();
        let params: HashMap<_, _> = url.query_pairs().collect::<HashMap<_, _>>();
        let mut scan_params = HashMap::new();
        let mut result: HashMap<String, Vec<Url>> = HashMap::new();
        let mut param_list = Vec::new();
        params.iter().for_each(|(key, value)| {
            scan_params.insert(key.to_string(), value.to_string());
            param_list.push(key.to_string());
        });
        drop(params);

        scan_params.iter().for_each(|(key, value)| {
            let mut edit_params = Vec::new();

            payload.split("\n").into_iter().for_each(|payload| {
                let mut new_params = scan_params.clone();
                new_params.insert(key.to_string(), value.as_str().to_owned() + payload);
                let mut new_url = url.clone();
                new_url.query_pairs_mut().clear();

                new_url.query_pairs_mut().extend_pairs(&new_params);

                edit_params.push(new_url);
            });

            result.insert(key.to_string(), edit_params);
        });
        result
    }
}
