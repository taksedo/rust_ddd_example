use std::{
    fmt::Debug,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use actix_web::web::Data;
use prettytable::{row, Table};
use usecase::main::menu::get_menu::GetMenu;

pub fn get_menu_command<T>(usecase: Data<Arc<Mutex<T>>>, mut stream: TcpStream)
where
    T: GetMenu + Send + Debug,
{
    let menu = usecase.lock().unwrap().execute();

    let mut table = Table::new();
    table.add_row(row!["Id", "Name", "Description", "Price"]);

    for meal_info in menu.iter() {
        table.add_row(row![
            meal_info.id.to_i64(),
            meal_info.name.to_string(),
            meal_info.description.to_string(),
            meal_info.price.to_string_value()
        ]);
    }

    let table_string = table.to_string();

    let _ = stream.write(table_string.as_bytes());

    // let _ = table.print(&mut stream.try_clone().unwrap());
    stream.flush().unwrap();
}
