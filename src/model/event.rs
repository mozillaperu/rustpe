use mysql as my;
use chrono;
use mysql::Pool as Pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    id: i32,
    name: String,
    about: String,
    date: chrono::NaiveDateTime
}

impl Event {

    pub fn all(pool: Pool) -> Vec<Event> {
        let selected_events: Vec<Event> =
        pool.prep_exec("SELECT id, name, about, date from heroku_c0585ee816e7fb3.event", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, about, date) = my::from_row(row);
                Event {
                    id: id,
                    name: name,
                    about: about,
                    date: date
                }
            }).collect()
        }).unwrap();
        return selected_events;
    }

}
