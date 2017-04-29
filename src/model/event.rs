use mysql as my;
use chrono;
use mysql::Pool as Pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    id: i32,
    name: String,
    about: String,
    date: chrono::NaiveDateTime,
    url: String,
    slide: String
}

impl Event {

    pub fn all(pool: Pool) -> Vec<Event> {
        let selected_events: Vec<Event> =
        pool.prep_exec("SELECT id, name, about, date, url, slide from heroku_c0585ee816e7fb3.event", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, about, date, url, slide) = my::from_row(row);
                Event {
                    id: id,
                    name: name,
                    about: about,
                    date: date,
                    url: url,
                    slide: slide
                }
            }).collect()
        }).unwrap();
        return selected_events;
    }

}
