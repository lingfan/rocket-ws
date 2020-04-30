use bson::ordered::OrderedDocument;
use sidekiq::{Client, ClientOpts, create_redis_pool, Job, JobOpts};

pub struct Notifier {
    client: Client
}

impl Notifier {
    pub fn new() -> Self {
        let client_opts = ClientOpts {
            namespace: None,
            ..Default::default()
        };

        //env::set_var("REDIS_NAMESPACE", config_rd.get_ns().as_str());
        //env::set_var("REDIS_URL_ENV", config_rd.get_uri().as_str());
        //let redis_url = env::var("REDIS_URL_ENV").unwrap().to_string();
        //let url = redis::parse_redis_url(&redis_url).unwrap();
        //let manager = RedisConnectionManager::new(url).unwrap();
        //let redis_pool = Pool::new(manager).map_err(|err| err.to_string());
        let redis_pool = create_redis_pool().unwrap();


        Notifier {
            client: Client::new(redis_pool, client_opts)
        }
    }

    pub fn connect(&self) {
        if let Err(err) = self.client.redis_pool.get() {
            panic!("Cannot create thread to RedisPool: {}", err)
        }
    }

    pub fn publish(&self, value: &OrderedDocument) {
        let job_args = json!(value);
        let job_options = JobOpts {
            queue: "notification".to_string(),
            ..Default::default()
        };

        let job = Job::new("ChatNotifier".to_string(), vec![job_args], job_options);

        if let Err(err) = self.client.push(job) {
            error!("SidekiqClient push failed: {}", err);
        }
    }
}
