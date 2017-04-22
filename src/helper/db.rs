use mysql as my;
use mysql::conn::{Opts, OptsBuilder};
use mysql::Pool as Pool;
use std::env;

pub struct DB {
}

impl DB {

    fn get_opts() -> Opts {
        let user = env::var("USER_DB").unwrap();
        let addr = env::var("ADDR_DB").unwrap();
        let pwd  = env::var("PWD_DB").unwrap();
        let port: u16 = 3306;
        let mut builder = OptsBuilder::default();
        builder.user(Some(user))
                .pass(Some(pwd))
                .ip_or_hostname(Some(addr))
                .tcp_port(port);
        return builder.into();
    }

    pub fn connection() -> Pool {
        return my::Pool::new(DB::get_opts()).unwrap();
    }

}
