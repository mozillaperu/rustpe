use mysql as my;
use mysql::Pool as Pool;

pub struct DB {
}

impl DB {
    pub fn connection() -> Pool {
        return my::Pool::new("mysql://root:1234@localhost:3306").unwrap();
    }
}
