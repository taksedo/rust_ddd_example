use std::{error::Error, fmt::Debug, sync::Arc};

use actix_web::web::Data;
use futures_util::SinkExt;
use nectar::{TelnetCodec, event::TelnetEvent};
use prettytable::{Table, row};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::Framed;
use usecase::menu::GetMenu;

pub async fn get_menu_command<T>(
    usecase: Data<Arc<Mutex<T>>>,
    frame: &mut Framed<TcpStream, TelnetCodec>,
) -> Result<(), Box<dyn Error>>
where
    T: GetMenu + Send + Debug,
{
    let menu = usecase.lock().await.execute().await;

    let mut table = Table::new();
    table.add_row(row!["Id", "Name", "Description", "Price"]);

    for meal_info in menu {
        table.add_row(row![
            meal_info.id.to_i64(),
            meal_info.name.to_string(),
            meal_info.description.to_string(),
            meal_info.price.to_string_value()
        ]);
    }

    let table_string = table.to_string();

    frame.send(TelnetEvent::Message(table_string)).await?;
    Ok(())
}
